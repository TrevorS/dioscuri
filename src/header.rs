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
    mime: Mime,
}

impl Header {
    pub fn new(status_code: StatusCode, mime: Mime) -> Self {
        Self { status_code, mime }
    }

    pub fn from(status_code: u8, mime_str: &str) -> Self {
        Self::new(status_code.into(), mime_str.parse().unwrap())
    }

    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    pub fn mime(&self) -> &Mime {
        &self.mime
    }
}

mod parser {
    use super::*;

    pub fn parse_success(header: &str) -> Header {
        let (_header, (status_code, mime_type)) = nom::sequence::separated_pair(
            status_code_digits,
            nom::bytes::complete::tag(" "),
            mime_type_str,
        )(header)
        .unwrap();

        let status_code = StatusCode::try_from(status_code).unwrap();
        let mime_type = mime_type.parse().unwrap();

        Header::new(status_code, mime_type)
    }

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
            let h = "20 text/gemini";

            assert_eq!(parse_success(&h).status_code(), StatusCode::Success);
            assert_eq!(parse_success(&h).mime().essence_str(), "text/gemini");
        }
    }
}
