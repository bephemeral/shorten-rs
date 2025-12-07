use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use url::{ParseError, Url};

#[derive(Deserialize)]
pub struct LinkNew {
    url: String,
}

impl LinkNew {
    pub fn parse(&self) -> Result<LinkNew, ()> {
        let link = match Url::parse(&self.url) {
            Ok(parsed) => {
                // allow only http and https
                match parsed.scheme() {
                    "http" | "https" => parsed.to_string(),
                    _ => return Err(()), // not really accurate but close enough
                }
            }
            Err(ParseError::RelativeUrlWithoutBase) => format!("http://{}", self.url),
            Err(_) => return Err(()),
        };

        Ok(LinkNew { url: link })
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Serialize, FromRow)]
pub struct Link {
    id: String,
    url: String,
}

impl Link {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
