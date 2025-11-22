use actix_web::web::{self, ServiceConfig};
use shorten_rs::{
    link::{create_link, get_link},
    state::AppState,
};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let state = web::Data::new(AppState::new());

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(state.clone())
            .service(get_link)
            .service(create_link);
    };

    Ok(config.into())
}
