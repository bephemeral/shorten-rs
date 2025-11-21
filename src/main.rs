use actix_web::{App, HttpServer, web};
use shorten_rs::{
    routes::link::{create_link, get_link},
    state::AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_link)
            .service(create_link)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
