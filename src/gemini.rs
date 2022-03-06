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

pub fn build_document(input: &[u8], url: &Url) -> Document {
    let lines = std::str::from_utf8(input).unwrap();

    let (_, document) = parser::parse(lines, url).unwrap();

    document
}

#[derive(Debug, Clone, PartialEq)]
pub enum Line {
    Text { content: String },
    Link { url: Url, link_name: Option<String> },
    PreformatToggle { alt_text: Option<String> },
    PreformattedText { content: String },
    Heading { content: String, level: u8 },
    UnorderedListItem { content: String },
    Quote { content: String },
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

    pub fn preformat_toggle(alt_text: Option<&str>) -> Self {
        Self::PreformatToggle {
            alt_text: alt_text.map(str::to_string),
        }
    }

    pub fn preformatted_text(content: &str) -> Self {
        Self::PreformattedText {
            content: content.to_string(),
        }
    }

    pub fn heading(content: &str, level: usize) -> Self {
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
    use nom::bytes::complete::{tag, take_while};
    use nom::character::complete::{line_ending, multispace0, not_line_ending};
    use nom::combinator::{all_consuming, map};
    use nom::multi::{many0, many1_count};
    use nom::sequence::{pair, preceded, terminated};
    use nom::IResult;

    pub fn parse<'a>(i: &'a str, url: &'a Url) -> IResult<&'a str, Document> {
        let mut preformatted = false;

        let (i, document) = all_consuming(map(
            many0(map(
                terminated(line(url), line_ending),
                |line: Line| match line {
                    Line::PreformatToggle { alt_text: _ } => {
                        preformatted = !preformatted;

                        line
                    }
                    Line::Text { ref content } => {
                        if preformatted {
                            // stop making two lines :(
                            // use a parser to make this work right
                            // maybe add attributes to regular text
                            Line::preformatted_text(content)
                        } else {
                            line
                        }
                    }
                    _ => line,
                },
            )),
            Document::new,
        ))(i)?;

        Ok((i, document))
    }

    fn line<'a>(url: &'a Url) -> impl FnMut(&'a str) -> IResult<&'a str, Line> {
        alt((
            link(url),
            preformat_toggle,
            heading,
            unordered_list_item,
            quote,
            text,
        ))
    }

    fn text(i: &str) -> IResult<&str, Line> {
        map(not_line_ending, Line::text)(i)
    }

    fn link<'a>(base_url: &'a Url) -> impl FnMut(&'a str) -> IResult<&'a str, Line> {
        map(
            preceded(
                terminated(tag("=>"), multispace0),
                pair(
                    take_while(is_valid_link_char),
                    map(not_line_ending, str_clean_up),
                ),
            ),
            |(url, name)| {
                // TODO: need to propagate errors properly
                let url = base_url.join(url).unwrap();

                Line::link(url, name)
            },
        )
    }

    fn preformat_toggle(i: &str) -> IResult<&str, Line> {
        map(
            preceded(tag("```"), map(not_line_ending, str_clean_up)),
            Line::preformat_toggle,
        )(i)
    }

    fn heading(i: &str) -> IResult<&str, Line> {
        map(
            pair(many1_count(tag("#")), map(not_line_ending, str::trim)),
            |(level, content)| Line::heading(content, level),
        )(i)
    }

    fn unordered_list_item(i: &str) -> IResult<&str, Line> {
        map(
            preceded(tag("*"), map(not_line_ending, str::trim)),
            Line::unordered_list_item,
        )(i)
    }

    fn quote(i: &str) -> IResult<&str, Line> {
        map(
            preceded(tag(">"), map(not_line_ending, str::trim)),
            Line::quote,
        )(i)
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
                Line::preformat_toggle(Some("python")),
                Line::preformatted_text("print('hello')"),
                Line::preformat_toggle(None),
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
        fn test_preformat_toggle() {
            let (_, actual) = preformat_toggle("```").unwrap();

            assert_eq!(Line::preformat_toggle(None), actual);
        }

        #[test]
        fn test_preformat_toggle_alt_text() {
            let (_, actual) = preformat_toggle("``` rust").unwrap();

            assert_eq!(Line::preformat_toggle(Some("rust")), actual)
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
        fn test_unordered_list_item() {
            let (_, actual) = unordered_list_item("* Example Unordered List Item").unwrap();

            assert_eq!(
                Line::unordered_list_item("Example Unordered List Item"),
                actual
            );
        }

        #[test]
        fn test_quote() {
            let (_, actual) = quote("> Example Quote").unwrap();

            assert_eq!(Line::quote("Example Quote"), actual);
        }
    }
}
