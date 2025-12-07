use crate::{
    link::{Link, LinkNew},
    state::AppState,
};
use actix_web::{HttpResponse, Responder, get, post, web};

#[get("/{id}")]
pub async fn get_link(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let Some(link) = data.redirects.get(&path.into_inner()) else {
        return HttpResponse::NotFound().finish();
    };

    HttpResponse::Found()
        .append_header(("Location", link.url()))
        .finish()
}

/*
#[post("/create")]
pub async fn create_link(payload: web::Json<LinkNew>, data: web::Data<AppState>) -> impl Responder {
    let link = match Link::new(&data, payload.into_inner()) {
        Ok(l) => l,
        Err(_) => return HttpResponse::BadRequest().body("Invalid URL"),
    };

    data.redirects.insert(link.id().to_string(), link);

    HttpResponse::Created().json(link)
}
*/
