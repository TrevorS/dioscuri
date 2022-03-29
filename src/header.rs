use core::fmt;

use anyhow::anyhow;
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

        #[allow(clippy::match_overlapping_arm)]
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

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    status: Status,
    inner: Inner,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Header status: {}, inner: {}", self.status, self.inner)
    }
}

#[allow(dead_code)]
impl Header {
    pub fn status(&self) -> Status {
        self.status
    }

    pub fn inner(&self) -> &Inner {
        &self.inner
    }

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

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub fn build_header(input: &[u8]) -> anyhow::Result<(Header, Option<Vec<u8>>)> {
    parser::parse(std::str::from_utf8(input)?)
        .map(|(body, header)| (header, Some(body.as_bytes().to_vec())))
        .map_err(|_| anyhow!("failed to parse bytes to utf8 in header"))
}

mod parser {
    use super::*;

    use Status::*;

    use nom::bytes::complete::{tag, take};
    use nom::character::complete::{line_ending, not_line_ending};
    use nom::combinator::map_res;
    use nom::sequence::{separated_pair, terminated};
    use nom::IResult;

    const SPACE: &str = " ";

    #[rustfmt::skip]
    pub fn parse(i: &str) -> IResult<&str, Header> {

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

    fn parse_gemini_header(i: &str) -> IResult<&str, (Status, &str)> {
        terminated(
            separated_pair(
                map_res(status_code_digits, Status::try_from),
                tag(SPACE),
                not_line_ending,
            ),
            line_ending,
        )(i)
    }

    fn status_code_digits(i: &str) -> IResult<&str, u8> {
        map_res(take(2usize), str::parse)(i)
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
