use url::Url;

#[derive(Debug, Clone)]
pub struct Settings {
    default_url: Url,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            default_url: "gemini://example.org".parse().unwrap(),
        }
    }

    pub fn default_url(&self) -> Option<Url> {
        Some(self.default_url.clone())
    }
}
