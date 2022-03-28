use std::fmt;
use std::rc::Rc;
use std::{io::Read, io::Write};

use log::info;
use url::Url;

use crate::response::Response;
use crate::tls::verification::{State, Verifier};
use crate::tls::{build_connector, get_stream};

pub struct GeminiClient {
    connector: native_tls::TlsConnector,
    verifier: Rc<dyn Verifier>,
}

impl fmt::Debug for GeminiClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        info!("getting url: {}", url.to_string());

        let mut stream = get_stream(&self.connector, url)?;

        let certificate = stream.peer_certificate()?;

        let certificate_status = self.verifier.verify(certificate.as_ref(), url)?;
        info!("TOFU certificate status: {}", certificate_status);

        // TODO: need to make it possible for a user to respond to this event in the UI
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
