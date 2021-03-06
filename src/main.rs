use actix_web::{App, HttpServer};

mod api;
mod common;
mod methods;
mod rules;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    log4rs::init_file("log/log4rs.yaml", Default::default()).unwrap();

    HttpServer::new(|| {
        App::new()
            .app_data(rules::Config::new())
            .service(api::uplink)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
