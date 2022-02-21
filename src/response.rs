use url::Url;

use crate::header::{Header, StatusCode};

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Ok,
}
#[derive(Debug, Clone)]
pub struct Response {
    header: Header,
    body: Option<Vec<u8>>,
    url: Url,
}

impl Response {
    pub fn parse(data: &[u8], url: &Url) -> Self {
        Self {
            header: Header::from(20, "text/gemini"),
            body: Some(data.to_vec()),
            url: url.to_owned(),
        }
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
