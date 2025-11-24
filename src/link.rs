use url::{ParseError, Url};

pub struct Link {
    url: String,
}

impl Link {
    pub fn new(url: &str) -> Result<Link, ()> {
        let link = match Url::parse(url) {
            Ok(parsed) => {
                // allow only http and https
                match parsed.scheme() {
                    "http" | "https" => parsed.into(),
                    _ => return Err(()), // not really accurate but close enough
                }
            }
            Err(ParseError::RelativeUrlWithoutBase) => {
                format!("http://{}", url)
            }
            Err(e) => return Err(()),
        };

        Ok(Link { url: link })
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }
}
