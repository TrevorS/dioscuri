use std::{io::Read, io::Write, net::ToSocketAddrs};

use url::Url;

use crate::response::Response;

pub struct GeminiClient {
    connector: native_tls::TlsConnector,
}

impl GeminiClient {
    pub fn new() -> Self {
        Self {
            connector: native_tls::TlsConnector::builder()
                .disable_built_in_roots(true)
                .danger_accept_invalid_certs(true)
                .build()
                .expect("Failed to create valid TlsConnector"),
        }
    }

    pub fn get(&self, url: &Url) -> Response {
        let host = url.host_str().unwrap();
        let port = url.port().unwrap_or(1965);

        let addr = (host, port).to_socket_addrs().unwrap().next().unwrap();

        let stream = std::net::TcpStream::connect(&addr).unwrap();

        let mut stream = self.connector.connect(host, stream).unwrap();

        let _certificate = stream.peer_certificate().unwrap().unwrap();

        stream
            .write_all(format!("{}\r\n", url.as_str()).as_bytes())
            .unwrap();

        let mut buf = vec![];
        stream.read_to_end(&mut buf).unwrap();

        Response::parse(&buf, url)
    }
}
