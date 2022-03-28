use std::rc::Rc;
use std::{io::Read, io::Write};

use url::Url;

use crate::response::Response;
use crate::tls::verification::{State, Verifier};
use crate::tls::{build_connector, get_stream};

pub struct GeminiClient {
    connector: native_tls::TlsConnector,
    verifier: Rc<dyn Verifier>,
}

impl std::fmt::Debug for GeminiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("GeminiClient")
    }
}

impl GeminiClient {
    pub fn new(verifier: Rc<dyn Verifier>) -> anyhow::Result<Self> {
        Ok(Self {
            connector: build_connector()?,
            verifier,
        })
    }

    pub fn get(&self, url: &Url) -> anyhow::Result<Response> {
        let mut stream = get_stream(&self.connector, url)?;

        let certificate = stream.peer_certificate()?;

        let certificate_status = self.verifier.verify(certificate.as_ref(), url)?;
        dbg!(&certificate_status);

        anyhow::ensure!(
            State::Conflict != certificate_status,
            "certificate conflict"
        );

        stream.write_all(format!("{}\r\n", url.as_str()).as_bytes())?;
        stream.flush()?;

        let mut buf = vec![];
        stream.read_to_end(&mut buf)?;

        Response::parse(&buf, url)
    }
}
