use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Link {
    url: String,
}

impl Link {
    pub fn new(url: &str) -> Link {
        Link {
            url: String::from(url),
        }
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }
}
