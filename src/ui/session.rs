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
    current: usize,
}

// TODO: is there a way to ensure that current + pages are always in sync (no bad states)
impl SessionHistory {
    pub fn new() -> Self {
        Self {
            pages: vec![],
            current: 0,
        }
    }

    pub fn go_back(&mut self) -> Option<&Page> {
        if !self.can_go_backward() {
            return None;
        }

        self.current -= 1;

        Some(&self.pages[self.current])
    }

    pub fn go_forward(&mut self) -> Option<&Page> {
        if !self.can_go_forward() {
            return None;
        }

        self.current += 1;

        Some(&self.pages[self.current])
    }

    pub fn can_go_forward(&self) -> bool {
        // not on the last element & the history needs to have more than 1 url in it
        self.pages.len() > 1 && self.current < self.pages.len() - 1
    }

    pub fn can_go_backward(&self) -> bool {
        // not on the first element & the history
        self.pages.len() > 1 && self.current > 0
    }

    pub fn navigate(&mut self, new_url: &str) {
        if self.can_go_forward() {
            let page = &self.pages[self.current + 1];

            if new_url == page.url() {
                self.current += 1;

                return;
            }
        }

        self.pages.truncate(self.current + 1);
        self.pages.push(new_url.into());

        self.current = self.pages.len() - 1;
    }
}
