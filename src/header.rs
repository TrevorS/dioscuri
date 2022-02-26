use mime::Mime;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
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

impl TryFrom<&str> for Status {
    type Error = std::num::ParseIntError;

    fn try_from(status: &str) -> Result<Self, Self::Error> {
        let status: u8 = str::parse(status)?;

        Ok(status.into())
    }
}

impl From<u8> for Status {
    fn from(status: u8) -> Self {
        use Status::*;

        match status {
            // specific errors
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

            // error ranges
            10..=19 => Input,
            20..=29 => Success,
            30..=39 => RedirectTemporary,
            40..=49 => TemporaryFailure,
            50..=59 => PermanentFailure,
            60..=69 => ClientCertificateRequired,

            // catch all
            unknown => Unknown(unknown),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    status: Status,
    inner: Inner,
}

impl Header {
    pub fn status(&self) -> Status {
        self.status
    }

    pub fn inner(&self) -> &Inner {
        &self.inner
    }
    // TODO: assert on status code range to ensure validity of combo
    pub fn input(status: Status, prompt: &str) -> Self {
        Self {
            status,
            inner: Inner::Input {
                prompt: Self::prepare_str(prompt),
            },
        }
    }

    pub fn success(status: Status, mime: Mime) -> Self {
        Self {
            status,
            inner: Inner::Success { mime },
        }
    }

    pub fn redirect(status: Status, url: url::Url) -> Self {
        Self {
            status,
            inner: Inner::Redirect { url },
        }
    }

    pub fn failure(status: Status, error: &str) -> Self {
        Self {
            status,
            inner: Inner::Failure {
                error: Self::prepare_str(error),
            },
        }
    }

    pub fn client_certificate(status: Status, error: &str) -> Self {
        Self {
            status,
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
        use Status::*;

        let (rest, (status, meta)) = parse_gemini_header(i)?;

        Ok((
            rest,
            match status {
                Input | InputSensitive => {
                    Header::input(status, meta)
                }
                Success => {
                    Header::success(status, meta.parse().unwrap())
                }
                RedirectTemporary | RedirectPermanent => {
                    Header::redirect(status, meta.parse().unwrap())
                }
                TemporaryFailure | ServerUnavailable | CgiError | ProxyError | SlowDown | PermanentFailure | NotFound | Gone | ProxyRequestRefused | BadRequest => {
                    Header::failure(status, meta)
                }
                ClientCertificateRequired | CertificateNotAuthorized | CertificateNotValid => {
                    Header::client_certificate(status, meta)
                }
                Unknown(status) => {
                    panic!("Unknown status code: {} | meta: {}", status, meta)
                }
            },
        ))
    }

    fn parse_gemini_header(i: &str) -> nom::IResult<&str, (Status, &str)> {
        nom::sequence::terminated(
            nom::sequence::separated_pair(
                nom::combinator::map_res(status_code_digits, Status::try_from),
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
        fn test_parse_success() {
            match parse("20 text/gemini\r\n").unwrap().1 {
                Header {
                    status,
                    inner: Inner::Success { mime },
                } => {
                    assert_eq!(status, Status::Success);
                    assert_eq!(mime.essence_str(), "text/gemini");
                }
                _ => unreachable!(),
            }
        }

        #[test]
        fn test_parse_success_range() {
            match parse("25 text/gemini\r\n").unwrap().1 {
                Header {
                    status,
                    inner: Inner::Success { mime },
                } => {
                    assert_eq!(status, Status::Success);
                    assert_eq!(mime.essence_str(), "text/gemini");
                }
                _ => unreachable!(),
            }
        }

        #[test]
        fn test_parse_input() {
            match parse("10 What is your name?\r\n").unwrap().1 {
                Header {
                    status,
                    inner: Inner::Input { prompt },
                } => {
                    assert_eq!(status, Status::Input);
                    assert_eq!(prompt, Some("What is your name?".to_string()));
                }
                _ => unreachable!(),
            }
        }

        #[test]
        fn test_parse_input_sensitive() {
            match parse("11 Would you like to play a game?\r\n").unwrap().1 {
                Header {
                    status,
                    inner: Inner::Input { prompt },
                } => {
                    assert_eq!(status, Status::InputSensitive);
                    assert_eq!(prompt, Some("Would you like to play a game?".to_string()));
                }
                _ => unreachable!(),
            }
        }
    }
}