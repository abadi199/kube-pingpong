extern crate actix_web;
extern crate reqwest;
#[macro_use]
extern crate failure;
extern crate serde;
use actix_web::*;

#[derive(Fail, Debug)]
enum PingPongError {
    #[fail(display = "Error connecting to Ping server")]
    PingError(reqwest::Error),
    #[fail(display = "Error reading json from Ping server")]
    PingJsonError(reqwest::Error),
    #[fail(display = "Error connecting to Pong server")]
    PongError(reqwest::Error),
    #[fail(display = "Error reading json from Pong server")]
    PongJsonError(reqwest::Error),
}

impl error::ResponseError for PingPongError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            PingPongError::PingError(_) => HttpResponse::new(http::StatusCode::BAD_REQUEST),
            PingPongError::PongError(_) => HttpResponse::new(http::StatusCode::BAD_REQUEST),
            PingPongError::PongJsonError(_) => HttpResponse::new(http::StatusCode::BAD_REQUEST),
            PingPongError::PingJsonError(_) => HttpResponse::new(http::StatusCode::BAD_REQUEST),
        }
    }
}

fn call_pong() -> Result<String, PingPongError> {
    // let resp: String = reqwest::get("http://10.0.206.143")
    let resp: String = reqwest::get("http://pong-service.default.svc.cluster.local")
        .map_err(PingPongError::PongError)?
        .text()
        .map_err(PingPongError::PongJsonError)?;
    Ok(resp)
}

fn call_ping() -> Result<String, PingPongError> {
    // let resp: String = reqwest::get("http://10.0.246.220")
    let resp: String = reqwest::get("http://ping-service.default.svc.cluster.local")
        .map_err(PingPongError::PingError)?
        .text()
        .map_err(PingPongError::PingJsonError)?;
    Ok(resp)
}

fn pingpong(_req: &HttpRequest) -> Result<String> {
    let ping: String = call_ping().unwrap_or("Ping is down".to_string());
    let pong = call_pong().unwrap_or("Pong is down".to_string());
    Ok(format!("{} -- {}", ping, pong))
}

fn alive(_req: &HttpRequest) -> impl Responder {
    format!("OK")
}

fn main() {
    println!("Starting Ping Ping server on port 8003");
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(pingpong))
            .resource("/alive", |r| r.f(alive))
    })
    .bind("0.0.0.0:8003")
    .expect("Can not bind to port 8003")
    .run();
}
