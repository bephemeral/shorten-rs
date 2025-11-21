use crate::state::AppState;
use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;

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

#[get("/link/{id}")]
pub async fn get_link(path: web::Path<usize>, data: web::Data<AppState>) -> impl Responder {
    let Some(link) = data.redirects.get(&path.into_inner()) else {
        return HttpResponse::NotFound().finish();
    };

    HttpResponse::Found()
        .append_header(("Location", link.url()))
        .finish()
}

#[post("/create")]
pub async fn create_link(link: web::Json<Link>, data: web::Data<AppState>) -> impl Responder {
    let current_id = data.last_id.fetch_add(1, Ordering::SeqCst);
    data.redirects.insert(current_id, link.into_inner());

    HttpResponse::Ok().json(Link::new(
        format!("http://localhost:8080/link/{}", current_id).as_str(),
    ))
}
