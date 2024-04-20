use actix_web::{error::ResponseError, HttpResponse};
use actix_web::{get, rt::task::spawn_blocking, web, App, HttpServer, Responder, Result};
use derive_more::Display; // Crate to easily implement display traits
use log::{error, info};

use std::{io::Error, sync::Mutex, time::Duration};

#[derive(Debug, Display)]
pub enum MyError {
    #[display(fmt = "Internal Server Error")]
    ServerError,
    #[display(fmt = "Overflow occurred during calculation")]
    OverflowError,
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::ServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            MyError::OverflowError => {
                HttpResponse::BadRequest().json("Overflow occurred during calculation")
            }
        }
    }
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/users/{user_id}/{friend}")]
async fn count_and_respond(
    data: web::Data<AppStateWithCounter>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse> {
    let (user_id, friend) = path.into_inner();
    info!("{user_id},{friend}"); // Using structured logging

    let mut counter = data.counter.lock().map_err(|e| MyError::ServerError)?;

    *counter += 1;
    info!("Request number: {counter}");

    let result = spawn_blocking(heavy_work)
        .await
        .map_err(|_| MyError::ServerError)??;
    Ok(HttpResponse::Ok().body(format!(
        "Request number: {counter} {:?} | {user_id} and {friend} are friends",
        result
    )))
}

fn heavy_work() -> Result<u128, MyError> {
    let mut cumt: u128 = 1;
    for i in 1..10 {
        cumt = cumt.checked_mul(i).ok_or(MyError::OverflowError)?;
    }
    Ok(cumt)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(count_and_respond)
        // .route("/", web::get().to(count_and_respond))
    })
    .workers(2)
    .bind(("192.168.161.182", 8080))?
    .run()
    .await
}
