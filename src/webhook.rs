use crate::rules;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use log::info;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}")
}

#[post("/clean/{device_name}")]
pub async fn uplink(
    req: HttpRequest,
    post: String,
    device_name: web::Path<String>,
) -> impl Responder {
    info!("Request for {}", device_name);
    // Loads a vector of rules from the rules directory
    let config = match req.app_data::<Vec<rules::Config>>() {
        Some(c) => c,
        None => {
            return HttpResponse::BadRequest().body(format!("Error: Configuration file not found."))
        }
    };

    for c in config.into_iter() {
        if c.device_name == device_name.to_string() {
            return HttpResponse::Ok().body(format!("{}: {}", device_name, post));
        }
    }
    HttpResponse::BadRequest().body(format!("Error: \"{device_name}\" device not found."))
}
