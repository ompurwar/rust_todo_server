mod db;
mod todo_modal;
mod todos;

use actix_web::middleware::Logger;
use actix_web::post;
use actix_web::{error::ResponseError, HttpResponse};
use actix_web::{web, App, HttpServer, Result};
use db::create_db;
use derive_more::Display;
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, error};

use std::env;
use todos::{dto::CreateTodoDto, repository::TodoRepository};

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

#[post("todo")]
async fn create_todo(
    db_client: web::Data<TodoRepository>,
    todo: web::Json<CreateTodoDto>,
) -> Result<HttpResponse> {
    debug!("Received TODO: {:#?}", todo);

    let result = db_client.save(&todo.into_inner().into()).await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("todo added")),
        Err(err) => {
            error!("{:#?}", err);
            Ok(HttpResponse::InternalServerError().body(err.to_string()))
        }
    }

    // let result = spawn_blocking(heavy_work)
    //     .await
    //     .map_err(|_| MyError::ServerError)??;
    //Ok(HttpResponse::Created().body(format!("Request processed ",)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let DB_NAME: String =
        env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set");
    let db = create_db(&DB_NAME).await;
    // let counter = web::Data::new(AppStateWithCounter {
    //     counter: Mutex::new(0),
    // });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            // .app_data(counter.clone())
            .app_data(web::Data::new(TodoRepository::new(&db)))
            .service(create_todo)
        // .route("/", web::get().to(count_and_respond))
    })
    .workers(2)
    .bind(("localhost", 8080))?
    .run()
    .await
}
