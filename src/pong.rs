extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder};

fn ping(_req: &HttpRequest) -> impl Responder {
    format!("Pong")
}

fn alive(_req: &HttpRequest) -> impl Responder {
    format!("OK")
}

fn main() {
    println!("Starting Pong server on port 8001");
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(ping))
            .resource("/alive", |r| r.f(alive))
    })
    .bind("0.0.0.0:8001")
    .expect("Can not bind to port 8001")
    .run();
}
