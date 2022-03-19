use crate::{methods, rules};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use log::info;
use serde_json::Value;
use std::collections::HashMap;

#[post("/clean/{device_name}")]
pub async fn uplink(
    req: HttpRequest,
    post: String,
    device_name: web::Path<String>,
) -> impl Responder {
    info!("[POST]: For {}", device_name);
    // Loads a vector of rules from the rules directory
    let config = match req.app_data::<Vec<rules::Config>>() {
        Some(c) => c,
        None => {
            return HttpResponse::NotFound().body(format!("Error: Configuration file not found."))
        }
    };

    // Creates a variables map that can be passed to various cleaning methods
    let variables_map: HashMap<String, Value> = match serde_json::from_str(&post.to_lowercase()) {
        Ok(vm) => vm,
        Err(e) => {
            return HttpResponse::BadRequest()
                .body(format!("Error: Unable to parse provided json. {}", e))
        }
    };

    // Convert from  Value to String (do this once then compare below)
    let dev_name = device_name.to_string();

    // Loop through the config vector to find the matching device, returns 400 if device
    // cannot be found
    for c in config.into_iter() {
        if c.device_name == dev_name {
            match methods::bounds::limits(&dev_name, &variables_map, c) {
                Ok(res) => {
                    match serde_json::to_string(&res) {
                        Ok(r) => {
                            return HttpResponse::Ok()
                                .insert_header(("ContentType", "application/json"))
                                .body(r)
                        }
                        Err(e) => {
                            return HttpResponse::BadRequest()
                                .body(format!("Error: Unable to parse response json. {}", e))
                        }
                    };
                }
                Err(e) => {
                    return HttpResponse::BadRequest()
                        .body(format!("Error: Unable to parse provided json. {}", e))
                }
            }
        }
    }
    HttpResponse::NotFound().body(format!("Error: \"{device_name}\" device not found."))
}
