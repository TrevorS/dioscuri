use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

use native_tls::{TlsConnector, TlsStream};
use url::Url;

const DEFAULT_GEMINI_PORT: u16 = 1965;

pub fn build_connector() -> anyhow::Result<TlsConnector> {
    native_tls::TlsConnector::builder()
        .disable_built_in_roots(true)
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|_| anyhow::anyhow!("failed to build connector"))
}

pub fn get_stream(connector: &TlsConnector, url: &Url) -> anyhow::Result<TlsStream<TcpStream>> {
    let (host, addr) = url_to_socket_addrs(&url)?;
    let stream = TcpStream::connect(&addr)?;

    connector
        .connect(host, stream)
        .map_err(|_| anyhow::anyhow!("failed to connect to addr: {}", addr))
}

fn url_to_socket_addrs(url: &Url) -> anyhow::Result<(&str, SocketAddr)> {
    let host = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("could not extract host from url"))?;

    let port = url.port().unwrap_or(DEFAULT_GEMINI_PORT);

    let addrs = (host, port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| anyhow::anyhow!("failed to create SocketAddr"))?;

    Ok((host, addrs))
}

pub mod tofu {
    use native_tls::Certificate;
    use url::Url;

    #[derive(Debug, Clone, Copy)]
    pub enum State {
        NewCertificate,
        Matched,
        Conflict,
    }

    pub fn verify(certificate: Option<&Certificate>, url: &Url) -> anyhow::Result<State> {
        anyhow::ensure!(certificate.is_some(), "failed to receive peer certificate");

        let certificate = certificate.unwrap();
        let dns_name = dns_name_from_url(url)?;

        verify_dns_name(certificate, &dns_name)?;
        check_validity(certificate)?;

        Ok(State::NewCertificate)
    }

    fn verify_dns_name(
        certificate: &Certificate,
        dns_name: &webpki::DnsNameRef,
    ) -> anyhow::Result<()> {
        let raw = certificate.to_der()?;
        let certificate = webpki::EndEntityCert::try_from(raw.as_slice())?;

        certificate
            .verify_is_valid_for_dns_name(*dns_name)
            .map_err(|_| anyhow::anyhow!("failed to validate certificate via dns name"))
    }

    fn check_validity(certificate: &Certificate) -> anyhow::Result<()> {
        let raw = certificate.to_der()?;

        let (_, certificate) = x509_parser::parse_x509_certificate(&raw)?;

        certificate
            .validity()
            .is_valid()
            .then(|| {})
            .ok_or_else(|| anyhow::anyhow!("failed to validate certificate via time"))
    }

    fn dns_name_from_url(url: &Url) -> anyhow::Result<webpki::DnsNameRef> {
        webpki::DnsNameRef::try_from_ascii_str(
            url.host_str()
                .ok_or_else(|| anyhow::anyhow!("failed to convert url to ascii string"))?,
        )
        .map_err(|_| anyhow::anyhow!("failed to convert ascii string to dns name"))
    }
}
