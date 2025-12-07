use crate::state::AppState;
use actix_web::web::Data;
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use url::{ParseError, Url};

#[derive(Deserialize)]
pub struct LinkNew {
    url: String,
}

#[derive(Serialize, FromRow)]
pub struct Link {
    id: String,
    url: String,
}

fn get_unique_id(data: &Data<AppState>) -> String {
    let id = Alphanumeric.sample_string(&mut rand::rng(), 4);

    if data.redirects.contains_key(&id) {
        get_unique_id(data)
    } else {
        id
    }
}

impl Link {
    pub fn new(data: &Data<AppState>, link: LinkNew) -> Result<Link, ()> {
        let link = match Url::parse(&link.url) {
            Ok(parsed) => {
                // allow only http and https
                match parsed.scheme() {
                    "http" | "https" => parsed.to_string(),
                    _ => return Err(()), // not really accurate but close enough
                }
            }
            Err(ParseError::RelativeUrlWithoutBase) => format!("http://{}", link.url),
            Err(_) => return Err(()),
        };

        Ok(Link {
            id: get_unique_id(data),
            url: link,
        })
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }
}
