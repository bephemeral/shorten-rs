use crate::state::AppState;
use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[get("/{id}")]
pub async fn get_link(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let Ok(id) = Uuid::parse_str(path.into_inner().as_str()) else {
        return HttpResponse::BadRequest().finish();
    };
    let Some(link) = data.redirects.get(&id) else {
        return HttpResponse::NotFound().finish();
    };

    HttpResponse::Found()
        .append_header(("Location", link.url()))
        .finish()
}

#[post("/create")]
pub async fn create_link(link: web::Json<Link>, data: web::Data<AppState>) -> impl Responder {
    let id = Uuid::now_v7();

    data.redirects.insert(id, link.into_inner());

    HttpResponse::Ok().json(id)
}
