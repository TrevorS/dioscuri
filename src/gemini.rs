use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    lines: Vec<Line>,
}

impl Document {
    pub fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }

    pub fn lines(&self) -> &Vec<Line> {
        &self.lines
    }
}

pub fn build_document(input: &[u8], url: &Url) -> anyhow::Result<Document> {
    parser::parse(std::str::from_utf8(input)?, url)
        .map(|(_, d)| d)
        .map_err(|_| anyhow::anyhow!("failed to parse gemini document"))
}

#[derive(Debug, Clone, PartialEq)]
pub enum Line {
    Text {
        content: String,
    },
    Link {
        url: Url,
        link_name: Option<String>,
    },
    Preformatted {
        alt_text: Option<String>,
        lines: Vec<Line>,
    },
    Heading {
        content: String,
        level: u8,
    },
    UnorderedListItem {
        content: String,
    },
    Quote {
        content: String,
    },
}

impl Line {
    pub fn text(content: &str) -> Self {
        Self::Text {
            content: content.to_string(),
        }
    }

    pub fn link(url: Url, link_name: Option<&str>) -> Self {
        Self::Link {
            url,
            link_name: link_name.map(str::to_string),
        }
    }

    pub fn preformatted(alt_text: Option<&str>, lines: Vec<Line>) -> Self {
        Self::Preformatted {
            alt_text: alt_text.map(str::to_string),
            lines,
        }
    }

    pub fn heading(content: &str, level: usize) -> Self {
        // maybe we log here and clamp between 1 and 3
        // to avoid possible error
        assert!(level > 0 && level <= 3);

        Self::Heading {
            content: content.to_string(),
            level: level.try_into().unwrap(),
        }
    }

    pub fn unordered_list_item(content: &str) -> Self {
        Self::UnorderedListItem {
            content: content.to_string(),
        }
    }

    pub fn quote(content: &str) -> Self {
        Self::Quote {
            content: content.to_string(),
        }
    }
}

mod parser {
    use super::*;

    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_until, take_while};
    use nom::character::complete::{line_ending, multispace0, not_line_ending};
    use nom::combinator::{all_consuming, map, map_res, opt};
    use nom::multi::{many0, many1_count};
    use nom::sequence::{delimited, pair, preceded, terminated};
    use nom::IResult;

    const LINK_ARROW: &str = "=>";
    const LIST_STAR: &str = "*";
    const QUOTE_ARROW: &str = ">";
    const HEADER_OCTOTHORPE: &str = "#";
    const PREFORMAT_PREFIX: &str = "```";

    pub fn parse<'a>(i: &'a str, base_url: &'a Url) -> IResult<&'a str, Document> {
        map(
            all_consuming(many0(terminated(line(base_url), line_ending))),
            Document::new,
        )(i)
    }

    fn line<'a>(base_url: &'a Url) -> impl FnMut(&'a str) -> IResult<&'a str, Line> {
        alt((
            link(base_url),
            preformatted,
            heading,
            simple_line(LIST_STAR, &Line::unordered_list_item),
            simple_line(QUOTE_ARROW, &Line::quote),
            text,
        ))
    }

    fn text(i: &str) -> IResult<&str, Line> {
        map(not_line_ending, Line::text)(i)
    }

    fn link<'a>(base_url: &'a Url) -> impl FnMut(&'a str) -> IResult<&'a str, Line> {
        map_res::<_, _, _, _, nom::Err<url::ParseError>, _, _>(
            preceded(
                terminated(tag(LINK_ARROW), multispace0),
                pair(
                    take_while(is_valid_link_char),
                    map(not_line_ending, str_clean_up),
                ),
            ),
            |(url, name)| {
                base_url
                    .join(url)
                    .map(|url| Line::link(url, name))
                    .map_err(nom::Err::Error)
            },
        )
    }

    fn preformatted(i: &str) -> IResult<&str, Line> {
        map(
            pair(preformat_header, preformat_body),
            |(alt_text, lines)| Line::preformatted(alt_text, lines),
        )(i)
    }

    fn preformat_header(i: &str) -> IResult<&str, Option<&str>> {
        map(
            delimited(tag(PREFORMAT_PREFIX), opt(not_line_ending), line_ending),
            |alt_text| alt_text.map(str::trim),
        )(i)
    }

    fn preformat_body(i: &str) -> IResult<&str, Vec<Line>> {
        map(
            terminated(
                map_res(
                    take_until(PREFORMAT_PREFIX),
                    many0(terminated(text, line_ending)),
                ),
                tag(PREFORMAT_PREFIX),
            ),
            |(_, lines)| lines,
        )(i)
    }

    fn heading(i: &str) -> IResult<&str, Line> {
        map(
            pair(
                many1_count(tag(HEADER_OCTOTHORPE)),
                map(not_line_ending, str::trim),
            ),
            |(level, content)| Line::heading(content, level),
        )(i)
    }

    fn simple_line<'a>(
        prefix: &'a str,
        constructor: &'a dyn Fn(&str) -> Line,
    ) -> impl FnMut(&'a str) -> IResult<&str, Line> {
        map(
            preceded(tag(prefix), map(not_line_ending, str::trim)),
            constructor,
        )
    }

    // TODO: improve this
    fn is_valid_link_char(c: char) -> bool {
        c.is_alphanumeric() || c.is_ascii_punctuation()
    }

    // TODO: improve this
    fn str_clean_up(i: &str) -> Option<&str> {
        if i.is_empty() {
            None
        } else {
            Some(i.trim())
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        fn example_dot_org() -> Url {
            Url::parse("gemini://example.org").unwrap()
        }

        fn parse_with_example_url(body: &str) -> Document {
            parse(body, &example_dot_org()).unwrap().1
        }

        fn line_with_example_url(line_str: &str) -> Line {
            line(&example_dot_org())(line_str).unwrap().1
        }

        #[test]
        fn test_parse_preformatted_text() {
            let actual = parse_with_example_url(
                "Hello Text!\r\n``` python\r\nprint('hello')\r\n```\r\nHello Text!\r\n",
            );

            let expected = Document::new(vec![
                Line::text("Hello Text!"),
                Line::preformatted(Some("python"), vec![Line::text("print('hello')")]),
                Line::text("Hello Text!"),
            ]);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_parse_text_quote_link() {
            let actual = parse_with_example_url(
                "Hello Text!\r\n> Hello Quote!\r\n=> gemini://example.org Example Link!\r\n",
            );

            let expected = Document::new(vec![
                Line::text("Hello Text!"),
                Line::quote("Hello Quote!"),
                Line::link(example_dot_org(), Some("Example Link!")),
            ]);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_line_text() {
            assert_eq!(
                Line::text("Hello line!"),
                line_with_example_url("Hello line!")
            )
        }

        #[test]
        fn test_line_quote() {
            assert_eq!(
                Line::quote("Hello quote!"),
                line_with_example_url("> Hello quote!"),
            );
        }

        #[test]
        fn test_text() {
            let (_, actual) = text("Hello world").unwrap();

            assert_eq!(Line::text("Hello world"), actual);
        }

        #[test]
        fn test_link() {
            let (_, actual) =
                link(&Url::parse("gemini://example.org").unwrap())("=> gemini://example.org")
                    .unwrap();

            assert_eq!(
                Line::link("gemini://example.org".parse().unwrap(), None),
                actual
            );
        }

        #[test]
        fn test_link_relative() {
            let (_, actual) =
                link(&Url::parse("gemini://example.org").unwrap())("=> /file.gmi Example Link")
                    .unwrap();

            assert_eq!(
                Line::link(
                    "gemini://example.org/file.gmi".parse().unwrap(),
                    Some("Example Link")
                ),
                actual
            );
        }

        #[test]
        fn test_header_1() {
            let (_, actual) = heading("# Example").unwrap();

            assert_eq!(Line::heading("Example", 1), actual);
        }

        #[test]
        fn test_header_2() {
            let (_, actual) = heading("## Example 2").unwrap();

            assert_eq!(Line::heading("Example 2", 2), actual);
        }

        #[test]
        #[should_panic]
        fn test_header_panic() {
            heading("#### Example").unwrap();
        }

        #[test]
        fn test_simple_line_unordered_list_item() {
            let (_, actual) =
                simple_line("*", &Line::unordered_list_item)("* Example Unordered List Item")
                    .unwrap();

            assert_eq!(
                Line::unordered_list_item("Example Unordered List Item"),
                actual
            );
        }

        #[test]
        fn test_simple_line_quote() {
            let (_, actual) = simple_line(">", &Line::quote)("> Example Quote").unwrap();

            assert_eq!(Line::quote("Example Quote"), actual);
        }
    }
}
