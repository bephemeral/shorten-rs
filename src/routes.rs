use crate::{link::Link, state::AppState};
use actix_web::{HttpResponse, Responder, get, post, web};
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LinkPayload {
    url: String,
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
