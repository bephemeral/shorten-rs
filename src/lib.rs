use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

pub struct AppState {
    redirects: Mutex<HashMap<i32, Link>>,
    last_id: Mutex<i32>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            redirects: Mutex::new(HashMap::new()),
            last_id: Mutex::new(0),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Link {
    url: String,
}

#[get("/link/{id}")]
pub async fn get_link(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let Ok(redirects) = data.redirects.lock() else {
        return HttpResponse::InternalServerError().finish();
    };
    let Some(link) = redirects.get(&id) else {
        return HttpResponse::NotFound().finish();
    };

    HttpResponse::Found()
        .append_header(("Location", link.url.as_str()))
        .finish()
}

#[post("/create")]
pub async fn create_link(link: web::Json<Link>, data: web::Data<AppState>) -> impl Responder {
    let Ok(mut redirects) = data.redirects.lock() else {
        return HttpResponse::InternalServerError().finish();
    };
    let Ok(mut last_id) = data.last_id.lock() else {
        return HttpResponse::InternalServerError().finish();
    };

    *last_id += 1;
    redirects.insert(*last_id, link.into_inner());

    HttpResponse::Ok().json(Link {
        url: format!("http://localhost:8080/link/{}", *last_id),
    })
}
