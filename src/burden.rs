use std::process;
use std::sync;
use actix_web::{web, error, Responder};

use crate::data;

pub fn put_a_burden(data: web::Data<sync::Mutex<data::Data>>) -> impl Responder {
    process::Command::new("dd")
        .arg("if=/dev/zero")
        .arg("of=/dev/null")
        // process::Command::new("yes")
        // .stdout(process::Stdio::null())
        .spawn()
        .map(|child| {
            let mut d = data.lock().unwrap();
            d.burden_process.push(child);
            "ok".to_string()
        }).map_err(|e| {
        error::ErrorInternalServerError(e)
    })
}

pub fn reduce_the_burden(data: web::Data<sync::Mutex<data::Data>>) -> impl Responder {
    let mut d = data.lock().unwrap();
    if let Some(mut burden) = d.burden_process.pop() {
        burden.kill().map(|()| {
            "Reduce the burden."
        }).map_err(|e| {
            error::ErrorInternalServerError(e)
        })
    } else {
        Ok("There is no burden.")
    }
}
