use url::Url;

pub struct Document {
    lines: Vec<Line>,
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

    use nom::bytes::complete::{tag, take_while};
    use nom::character::complete::{line_ending, multispace0, not_line_ending};
    use nom::combinator::map;
    use nom::multi::many1_count;
    use nom::sequence::{pair, preceded, terminated};
    use nom::IResult;

    fn text(i: &str) -> IResult<&str, Line> {
        let (i, content) = terminated(not_line_ending, line_ending)(i)?;

        Ok((i, Line::text(content)))
    }

    fn link(i: &str) -> IResult<&str, Line> {
        let (i, _) = terminated(tag("=>"), multispace0)(i)?;
        let (i, url) = take_while(is_valid_link_char)(i)?;
        let (i, name) = map(terminated(not_line_ending, line_ending), str_clean_up)(i)?;

        // TODO: convert error
        let url = Url::parse(url).unwrap();

        Ok((i, Line::link(url, name)))
    }

    fn preformat_toggle(i: &str) -> IResult<&str, Line> {
        let (i, (_, alt_text)) = pair(tag("```"), map(not_line_ending, str_clean_up))(i)?;

        Ok((i, Line::preformat_toggle(alt_text)))
    }

    fn heading(i: &str) -> IResult<&str, Line> {
        let (i, (level, content)) = terminated(
            pair(many1_count(tag("#")), map(not_line_ending, str::trim)),
            line_ending,
        )(i)?;

        Ok((i, Line::heading(content, level)))
    }

    fn unordered_list_item(i: &str) -> IResult<&str, Line> {
        let (i, content) = terminated(
            preceded(tag("*"), map(not_line_ending, str::trim)),
            line_ending,
        )(i)?;

        Ok((i, Line::unordered_list_item(content)))
    }

    fn quote(i: &str) -> IResult<&str, Line> {
        let (i, content) = terminated(
            preceded(tag(">"), map(not_line_ending, str::trim)),
            line_ending,
        )(i)?;

        Ok((i, Line::quote(content)))
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

        #[test]
        fn test_text() {
            let (_, actual) = text("Hello world\r\n").unwrap();

            assert_eq!(Line::text("Hello world"), actual);
        }

        #[test]
        fn test_link() {
            let (_, actual) = link("=> gemini://example.org\r\n").unwrap();

            assert_eq!(
                Line::link("gemini://example.org".parse().unwrap(), None),
                actual
            );
        }

        #[test]
        fn test_link_with_name() {
            let (_, actual) = link("=> gemini://example.org Example Link\r\n").unwrap();

            assert_eq!(
                Line::link(
                    "gemini://example.org".parse().unwrap(),
                    Some("Example Link")
                ),
                actual
            );
        }

        #[test]
        fn test_preformat_toggle() {
            let (_, actual) = preformat_toggle("```\r\n").unwrap();

            assert_eq!(Line::preformat_toggle(None), actual);
        }

        #[test]
        fn test_preformat_toggle_alt_text() {
            let (_, actual) = preformat_toggle("``` rust\r\n").unwrap();

            assert_eq!(Line::preformat_toggle(Some("rust")), actual)
        }

        #[test]
        fn test_header_1() {
            let (_, actual) = heading("# Example\r\n").unwrap();

            assert_eq!(Line::heading("Example", 1), actual);
        }

        #[test]
        fn test_header_2() {
            let (_, actual) = heading("## Example 2\r\n").unwrap();

            assert_eq!(Line::heading("Example 2", 2), actual);
        }

        #[test]
        #[should_panic]
        fn test_header_panic() {
            heading("#### Example\r\n").unwrap();
        }

        #[test]
        fn test_unordered_list_item() {
            let (_, actual) = unordered_list_item("* Example Unordered List Item\r\n").unwrap();

            assert_eq!(
                Line::unordered_list_item("Example Unordered List Item"),
                actual
            );
        }

        #[test]
        fn test_quote() {
            let (_, actual) = quote("> Example Quote\r\n").unwrap();

            assert_eq!(Line::quote("Example Quote"), actual);
        }
    }
}
