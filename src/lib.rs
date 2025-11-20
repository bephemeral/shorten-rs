use actix_web::{
    Responder, get, post,
    web::{self, Redirect},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

pub struct AppState {
    redirects: Mutex<HashMap<i32, String>>,
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
    let redirects = data.redirects.lock().unwrap();
    let link = redirects.get(&id).unwrap();

    Redirect::to(link.clone())
}

#[post("/create")]
pub async fn create_link(link: web::Json<Link>, data: web::Data<AppState>) -> web::Json<Link> {
    let mut redirects = data.redirects.lock().unwrap();
    let mut last_id = data.last_id.lock().unwrap();

    *last_id += 1;
    redirects.insert(*last_id, link.url.clone());

    web::Json(Link {
        url: format!("localhost:8080/link/{}", *last_id),
    })
}
