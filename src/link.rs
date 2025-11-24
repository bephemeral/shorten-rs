use crate::state::AppState;
use actix_web::{HttpResponse, Responder, get, post, web};
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

pub struct Link {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct LinkPayload {
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

fn get_unique_id(data: &web::Data<AppState>) -> String {
    let id = Alphanumeric.sample_string(&mut rand::rng(), 4);

    if data.redirects.contains_key(&id) {
        get_unique_id(data)
    } else {
        id
    }
}

#[get("/{id}")]
pub async fn get_link(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let Some(link) = data.redirects.get(&path.into_inner()) else {
        return HttpResponse::NotFound().finish();
    };

    HttpResponse::Found()
        .append_header(("Location", link.url()))
        .finish()
}

#[post("/create")]
pub async fn create_link(
    payload: web::Json<LinkPayload>,
    data: web::Data<AppState>,
) -> impl Responder {
    let link = match Link::new(&payload.url) {
        Ok(l) => l,
        Err(_) => return HttpResponse::BadRequest().body("Invalid URL"),
    };

    let id = get_unique_id(&data);

    data.redirects.insert(id.clone(), link);

    HttpResponse::Ok().json(id)
}
