use crate::{
    link::{Link, LinkNew},
    state::AppState,
};
use actix_web::{HttpResponse, get, post, web};

#[get("/{id}")]
pub async fn get_link(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let result = sqlx::query_as::<_, Link>("SELECT * FROM links WHERE id = $1")
        .bind(path.into_inner())
        .fetch_one(&data.pool)
        .await;

    match result {
        Ok(link) => HttpResponse::Found()
            .append_header(("Location", link.url()))
            .finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/create")]
pub async fn create_link(payload: web::Json<LinkNew>, data: web::Data<AppState>) -> HttpResponse {
    let Ok(link) = payload.into_inner().parse() else {
        return HttpResponse::BadRequest().body("Invalid URL");
    };

    let result = sqlx::query_as::<_, Link>("INSERT INTO links (url) VALUES ($1) RETURNING id, url")
        .bind(link.url())
        .fetch_one(&data.pool)
        .await;

    match result {
        Ok(link) => HttpResponse::Created().json(link),
        Err(e) => HttpResponse::InternalServerError().body(format!("database error {e}")),
    }
}
