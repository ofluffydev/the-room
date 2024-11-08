use std::collections::HashMap;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use confik::{Configuration as _, EnvSource};
use deadpool_postgres::{Client, Pool};
use dotenvy::dotenv;
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;

mod config;
mod db;
mod errors;
mod models;

use self::{errors::CustomErrors, models::Message};

static ABOUT_MESSAGE: &str = "Welcome to The Room! You can retrieve all messages by sending a GET request to /messages or add a new message by sending a POST request to /messages. Enjoy your stay!";

#[actix_web::get("/")]
async fn about() -> HttpResponse {
    HttpResponse::Ok().body(ABOUT_MESSAGE)
}

pub async fn get_messages(
    db_pool: web::Data<Pool>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(CustomErrors::PoolError)?;

    let last: Option<usize> = query.get("last").and_then(|val| val.parse().ok());

    let messages = db::get_messages(&client, last).await?;

    Ok(HttpResponse::Ok().json(messages))
}

pub async fn add_message(
    user: web::Json<Message>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: Message = user.into_inner();

    let client: Client = db_pool.get().await.map_err(CustomErrors::PoolError)?;

    // Attempt to add the message
    match db::add_message(&client, user_info).await {
        Ok(new_user) => {
            println!("Message added successfully: {:?}", new_user);
            Ok(HttpResponse::Ok().json(new_user))
        }
        Err(e) => {
            println!("Error adding message: {:?}", e);
            // Return appropriate error response
            Err(actix_web::error::ErrorNotFound(e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = ExampleConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/messages")
                    .route(web::post().to(add_message))
                    .route(web::get().to(get_messages)),
            )
            .service(about)
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
