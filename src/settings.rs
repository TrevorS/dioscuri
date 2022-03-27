use url::Url;

#[derive(Debug, Clone)]
pub struct Settings {
    default_url: Url,
    database_path: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            default_url: "gemini://gemini.conman.org".parse().unwrap(),
            database_path: "dioscuri.sqlite".to_string(),
        }
    }

    pub fn default_url(&self) -> Option<Url> {
        Some(self.default_url.clone())
    }

    pub fn database_path(&self) -> String {
        self.database_path.clone()
    }
}
