use actix_web::{HttpResponse, Responder};

pub fn healthz() -> impl Responder {
    HttpResponse::Ok().body("ok health")
}
