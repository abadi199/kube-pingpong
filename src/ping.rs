extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder};

fn ping(_req: &HttpRequest) -> impl Responder {
    format!("Ping")
}

fn alive(_req: &HttpRequest) -> impl Responder {
    format!("OK")
}

fn main() {
    println!("Starting Ping server on port 8000");
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(ping))
            .resource("/alive", |r| r.f(alive))
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run();
}
