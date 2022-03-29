use core::fmt;

#[derive(Debug, Clone)]
pub struct Page {
    url: String,
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.url)
    }
}

impl Page {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

impl From<&str> for Page {
    fn from(url: &str) -> Self {
        Self::new(url)
    }
}

#[derive(Debug, Clone)]
pub struct SessionHistory {
    pages: Vec<Page>,
    index: usize,
}

impl SessionHistory {
    pub fn new() -> Self {
        Self {
            pages: vec![],
            index: 0,
        }
    }

    pub fn go_back(&mut self) -> Option<&Page> {
        if self.pages.len() <= 1 {
            return None;
        }

        if self.index < 1 {
            return None;
        }

        self.index -= 1;

        Some(&self.pages[self.index])
    }

    pub fn go_forward(&mut self) -> Option<&Page> {
        if self.pages.len() <= 1 {
            return None;
        }

        if self.index >= self.pages.len() - 1 {
            return None;
        }

        self.index += 1;

        Some(&self.pages[self.index])
    }

    pub fn navigate(&mut self, url: &str) {
        self.pages.push(url.into());

        self.index = self.pages.len() - 1;
    }
}
