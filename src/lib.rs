use actix_web::{HttpResponse, Responder, get, post, web};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct AppState {
    redirects: DashMap<usize, Link>,
    last_id: AtomicUsize,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            redirects: DashMap::new(),
            last_id: AtomicUsize::new(0),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Link {
    url: String,
}

#[get("/link/{id}")]
pub async fn get_link(path: web::Path<usize>, data: web::Data<AppState>) -> impl Responder {
    let Some(link) = data.redirects.get(&path.into_inner()) else {
        return HttpResponse::NotFound().finish();
    };

    HttpResponse::Found()
        .append_header(("Location", link.url.as_str()))
        .finish()
}

#[post("/create")]
pub async fn create_link(link: web::Json<Link>, data: web::Data<AppState>) -> impl Responder {
    let current_id = data.last_id.fetch_add(1, Ordering::SeqCst);
    data.redirects.insert(current_id, link.into_inner());

    HttpResponse::Ok().json(Link {
        url: format!("http://localhost:8080/link/{}", current_id),
    })
}
