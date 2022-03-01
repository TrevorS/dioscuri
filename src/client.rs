use std::{io::Read, io::Write};

use url::Url;

use crate::response::Response;
use crate::tls::tofu::verify;
use crate::tls::{build_connector, get_stream};

pub struct GeminiClient {
    connector: native_tls::TlsConnector,
}

impl GeminiClient {
    pub fn new() -> Self {
        Self {
            connector: build_connector(),
        }
    }

    pub fn get(&self, url: &Url) -> anyhow::Result<Response> {
        let mut stream = get_stream(&self.connector, &url)?;

        let certificate = stream.peer_certificate()?;

        let certificate_status = verify(certificate.as_ref(), url)?;
        dbg!(&certificate_status);

        stream
            .write_all(format!("{}\r\n", url.as_str()).as_bytes())
            .unwrap();

        let mut buf = vec![];
        stream.read_to_end(&mut buf)?;

        Response::parse(&buf, url)
    }
}
