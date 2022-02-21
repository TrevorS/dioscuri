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
