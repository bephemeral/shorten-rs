use crate::state::AppState;
use actix_web::{HttpResponse, Responder, get, post, web};
use random_str::get_string;
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

fn get_unique_id(data: &web::Data<AppState>) -> String {
    let id = get_string(8, true, true, true, false);

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
pub async fn create_link(link: web::Json<Link>, data: web::Data<AppState>) -> impl Responder {
    let id = get_unique_id(&data);

    data.redirects.insert(id.clone(), link.into_inner());

    HttpResponse::Ok().json(id)
}
