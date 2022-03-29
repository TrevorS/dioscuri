#![allow(dead_code)]
use log::info;
use url::Url;

use crate::header::{build_header, Header};

#[derive(Debug, Clone)]
pub struct Response {
    header: Header,
    body: Option<Vec<u8>>,
    url: Url,
}

impl Response {
    pub fn parse(data: &[u8], url: &Url) -> anyhow::Result<Self> {
        let (header, body) = build_header(data)?;

        info!("parsed header: {}", &header);

        Ok(Self {
            header,
            body,
            url: url.to_owned(),
        })
    }

    pub fn body(&self) -> Option<&Vec<u8>> {
        self.body.as_ref()
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}
