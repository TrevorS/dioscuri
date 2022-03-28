use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

use anyhow::anyhow;
use native_tls::{TlsConnector, TlsStream};
use url::Url;

const DEFAULT_GEMINI_PORT: u16 = 1965;

pub fn build_connector() -> anyhow::Result<TlsConnector> {
    TlsConnector::builder()
        .disable_built_in_roots(true)
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|_| anyhow!("failed to build connector"))
}

pub fn get_stream(connector: &TlsConnector, url: &Url) -> anyhow::Result<TlsStream<TcpStream>> {
    let (host, addr) = url_to_socket_addrs(url)?;
    let stream = TcpStream::connect(&addr)?;

    connector
        .connect(host, stream)
        .map_err(|_| anyhow!("failed to connect to addr: {}", addr))
}

fn url_to_socket_addrs(url: &Url) -> anyhow::Result<(&str, SocketAddr)> {
    let host = url
        .host_str()
        .ok_or_else(|| anyhow!("could not extract host from url"))?;

    let port = url.port().unwrap_or(DEFAULT_GEMINI_PORT);

    let addrs = (host, port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| anyhow!("failed to create SocketAddr"))?;

    Ok((host, addrs))
}

pub mod verification {
    use std::fmt;

    use anyhow::anyhow;
    use native_tls::Certificate;
    use sha2::Digest;
    use url::Url;

    use crate::db::Db;

    #[derive(Debug, Clone, PartialEq)]
    pub enum State {
        New,
        Matched,
        Conflict,
    }

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub trait Verifier {
        fn verify(&self, certificate: Option<&Certificate>, url: &Url) -> anyhow::Result<State>;
    }

    pub struct TofuVerifier {
        db: Db,
    }

    impl TofuVerifier {
        pub fn new(db: Db) -> Self {
            Self { db }
        }
    }

    impl Verifier for TofuVerifier {
        fn verify(&self, certificate: Option<&Certificate>, url: &Url) -> anyhow::Result<State> {
            anyhow::ensure!(certificate.is_some(), "failed to receive peer certificate");

            let certificate = certificate.unwrap();
            let dns_name = dns_name_from_url(url)?;

            verify_dns_name(certificate, &dns_name)?;
            check_validity(certificate)?;

            let hostname = url
                .host_str()
                .ok_or_else(|| anyhow!("failed to extract host from url"))?;

            let fingerprint = create_fingerprint(certificate)?;

            match self.db.get_certificate(hostname)? {
                Some(existing) => {
                    if fingerprint == existing.fingerprint {
                        self.db
                            .update_certificate_timestamp(hostname)
                            .map(|_| Ok(State::Matched))?
                    } else {
                        Ok(State::Conflict)
                    }
                }
                None => self
                    .db
                    .insert_certificate(hostname, &fingerprint)
                    .map(|_| State::New),
            }
        }
    }

    fn verify_dns_name(
        certificate: &Certificate,
        dns_name: &webpki::DnsNameRef,
    ) -> anyhow::Result<()> {
        let raw = certificate.to_der()?;
        let certificate = webpki::EndEntityCert::try_from(raw.as_slice())?;

        certificate
            .verify_is_valid_for_dns_name(*dns_name)
            .map_err(|_| anyhow!("failed to validate certificate via dns name"))
    }

    fn check_validity(certificate: &Certificate) -> anyhow::Result<()> {
        let raw = certificate.to_der()?;

        let (_, certificate) = x509_parser::parse_x509_certificate(&raw)?;

        certificate
            .validity()
            .is_valid()
            .then(|| {})
            .ok_or_else(|| anyhow!("failed to validate certificate using time range validity"))
    }

    fn dns_name_from_url(url: &Url) -> anyhow::Result<webpki::DnsNameRef> {
        webpki::DnsNameRef::try_from_ascii_str(
            url.host_str()
                .ok_or_else(|| anyhow!("failed to convert url to ascii string"))?,
        )
        .map_err(|_| anyhow!("failed to convert ascii string to dns name"))
    }

    fn create_fingerprint(certificate: &Certificate) -> anyhow::Result<String> {
        let raw = certificate.to_der()?;

        Ok(base16ct::lower::encode_string(&sha2::Sha256::digest(raw)))
    }
}
