use std::env;
use actix_web::{web, error, HttpResponse, Responder};

pub fn set_env(path: web::Path<(String, String)>) -> impl Responder {
    println!("path: {:?}", path);
    env::set_var(&path.0, &path.1);
    HttpResponse::Ok().body("ok")
}

pub fn get_env(path: web::Path<String>) -> impl Responder {
    env::var(&path.into_inner())
        .map_err(|e| {
            error::ErrorInternalServerError(e)
        })
}
