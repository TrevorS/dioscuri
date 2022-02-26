use mime::Mime;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StatusCode {
    // 10
    Input,
    // 11
    InputSensitive,
    // 20
    Success,
    // 30
    RedirectTemporary,
    // 31
    RedirectPermanent,
    // 40
    TemporaryFailure,
    // 41
    ServerUnavailable,
    // 42
    CgiError,
    // 43
    ProxyError,
    // 44
    SlowDown,
    // 50
    PermanentFailure,
    // 51
    NotFound,
    // 52
    Gone,
    // 53
    ProxyRequestRefused,
    // 59
    BadRequest,
    // 60
    ClientCertificateRequired,
    // 61
    CertificateNotAuthorized,
    // 62
    CertificateNotValid,
    // ??
    Unknown(u8),
}

impl TryFrom<&str> for StatusCode {
    type Error = std::num::ParseIntError;

    fn try_from(status_code: &str) -> Result<Self, Self::Error> {
        let status_code: u8 = str::parse(status_code)?;

        Ok(status_code.into())
    }
}

impl From<u8> for StatusCode {
    // TODO: handle ranges between specific status codes
    fn from(status_code: u8) -> Self {
        use StatusCode::*;

        match status_code {
            10 => Input,
            11 => InputSensitive,
            20 => Success,
            30 => RedirectTemporary,
            31 => RedirectPermanent,
            40 => TemporaryFailure,
            41 => ServerUnavailable,
            42 => CgiError,
            43 => ProxyError,
            44 => SlowDown,
            50 => PermanentFailure,
            51 => NotFound,
            52 => Gone,
            53 => ProxyRequestRefused,
            59 => BadRequest,
            60 => ClientCertificateRequired,
            61 => CertificateNotAuthorized,
            62 => CertificateNotValid,
            unknown => Unknown(unknown),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    status_code: StatusCode,
    inner: Inner,
}

impl Header {
    // TODO: assert on status code range to ensure validity of combo
    pub fn input(status_code: StatusCode, prompt: &str) -> Self {
        Self {
            status_code,
            inner: Inner::Input {
                prompt: Self::prepare_str(prompt),
            },
        }
    }

    pub fn success(status_code: StatusCode, mime: Mime) -> Self {
        Self {
            status_code,
            inner: Inner::Success { mime },
        }
    }

    pub fn redirect(status_code: StatusCode, url: url::Url) -> Self {
        Self {
            status_code,
            inner: Inner::Redirect { url },
        }
    }

    pub fn failure(status_code: StatusCode, error: &str) -> Self {
        Self {
            status_code,
            inner: Inner::Failure {
                error: Self::prepare_str(error),
            },
        }
    }

    pub fn client_certificate(status_code: StatusCode, error: &str) -> Self {
        Self {
            status_code,
            inner: Inner::ClientCertificateRequired {
                error: Self::prepare_str(error),
            },
        }
    }

    fn prepare_str(s: &str) -> Option<String> {
        if s.is_empty() {
            None
        } else {
            Some(s.to_string())
        }
    }
}

#[derive(Debug, Clone)]
pub enum Inner {
    Input { prompt: Option<String> },
    Success { mime: Mime },
    Redirect { url: url::Url },
    Failure { error: Option<String> },
    ClientCertificateRequired { error: Option<String> },
}

pub fn build_header(input: &[u8]) -> (Header, Option<Vec<u8>>) {
    let input = std::str::from_utf8(input).unwrap();

    let (body, header) = parser::parse(input).unwrap();

    (header, Some(body.as_bytes().to_vec()))
}

mod parser {
    use super::*;

    #[rustfmt::skip]
    pub fn parse(i: &str) -> nom::IResult<&str, Header> {
        use StatusCode::*;

        let (rest, (status_code, meta)) = parse_gemini_header(i)?;

        Ok((
            rest,
            match status_code {
                Input | InputSensitive => {
                    Header::input(status_code, meta)
                }
                Success => {
                    Header::success(status_code, meta.parse().unwrap())
                }
                RedirectTemporary | RedirectPermanent => {
                    Header::redirect(status_code, meta.parse().unwrap())
                }
                TemporaryFailure | ServerUnavailable | CgiError | ProxyError | SlowDown | PermanentFailure | NotFound | Gone | ProxyRequestRefused | BadRequest => {
                    Header::failure(status_code, meta)
                }
                ClientCertificateRequired | CertificateNotAuthorized | CertificateNotValid => {
                    Header::client_certificate(status_code, meta)
                }
                Unknown(status_code) => {
                    panic!("Unknown status code: {} | meta: {}", status_code, meta)
                }
            },
        ))
    }

    fn parse_gemini_header(i: &str) -> nom::IResult<&str, (StatusCode, &str)> {
        nom::sequence::terminated(
            nom::sequence::separated_pair(
                nom::combinator::map_res(status_code_digits, StatusCode::try_from),
                nom::bytes::complete::tag(" "),
                nom::character::complete::not_line_ending,
            ),
            nom::character::complete::line_ending,
        )(i)
    }

    // helper parsers
    fn status_code_digits(i: &str) -> nom::IResult<&str, u8> {
        nom::combinator::map_res(nom::bytes::complete::take(2usize), str::parse)(i)
    }

    fn mime_type_str(i: &str) -> nom::IResult<&str, &str> {
        nom::bytes::complete::take_while(valid_mime_type_char)(i)
    }

    fn valid_mime_type_char(c: char) -> bool {
        c.is_alphanumeric() || c == '/'
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_status_code_digits() {
            assert_eq!(
                status_code_digits("20 text/gemini"),
                Ok((" text/gemini", 20))
            )
        }

        #[test]
        fn test_mime_type_str() {
            assert_eq!(
                mime_type_str("text/gemini\r\nbody"),
                Ok(("\r\nbody", "text/gemini",))
            )
        }

        #[test]
        fn test_parse_success() {
            match parse("20 text/gemini\r\n").unwrap().1 {
                Header {
                    status_code,
                    inner: Inner::Success { mime },
                } => {
                    assert_eq!(status_code, StatusCode::Success);
                    assert_eq!(mime.essence_str(), "text/gemini");
                }
                _ => unreachable!(),
            }
        }

        #[test]
        fn test_parse_input() {
            match parse("10 What is your name?\r\n").unwrap().1 {
                Header {
                    status_code,
                    inner: Inner::Input { prompt },
                } => {
                    assert_eq!(status_code, StatusCode::Input);
                    assert_eq!(prompt, Some("What is your name?".to_string()));
                }
                _ => unreachable!(),
            }
        }

        #[test]
        fn test_parse_input_sensitive() {
            match parse("11 Would you like to play a game?\r\n").unwrap().1 {
                Header {
                    status_code,
                    inner: Inner::Input { prompt },
                } => {
                    assert_eq!(status_code, StatusCode::InputSensitive);
                    assert_eq!(prompt, Some("Would you like to play a game?".to_string()));
                }
                _ => unreachable!(),
            }
        }
    }
}
