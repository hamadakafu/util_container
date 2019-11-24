use std::sync;
use actix_web::{web, App, HttpServer};

mod healthz;
mod env;
mod burden;
mod data;

fn main() {
    let data = web::Data::new(sync::Mutex::new(data::Data {
        burden_process: vec![],
    }));
    HttpServer::new(move || {
        App::new()
            .register_data(data.clone())
            .route("/healthz", web::get().to(healthz::healthz))
            .route("/set_env/{key}/{value}", web::get().to(env::set_env))
            .route("/get_env/{key}", web::get().to(env::get_env))
            .route("/burden/put", web::get().to(burden::put_a_burden))
            .route("/burden/reduce", web::get().to(burden::reduce_the_burden))
    })
        .bind("localhost:8080")
        .unwrap()
        .run()
        .unwrap();
}
