use actix_web::{web, App, Error, HttpResponse, HttpServer};
use confik::{Configuration as _, EnvSource};
use deadpool_postgres::{Client, Pool};
use dotenvy::dotenv;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;

mod config;
mod db;
mod errors;
mod models;

use self::{errors::CustomErrors, models::Message};

#[derive(RustEmbed)]
#[folder = "banned_words/"]
struct WordList;

pub struct ContentFilter {
    banned_words: HashSet<String>,
}

impl ContentFilter {
    pub fn new() -> Self {
        let mut banned_words = HashSet::new();
        for file in WordList::iter() {
            if let Some(content) = WordList::get(&file) {
                let words = std::str::from_utf8(content.data.as_ref()).unwrap();
                for word in words.lines() {
                    banned_words.insert(word.to_string());
                }
            }
        }
        ContentFilter { banned_words }
    }
}

impl ContentFilter {
    pub fn filter_text(&self, text: &str) -> String {
        let mut filtered = text.to_string();
        for word in &self.banned_words {
            let replacement = "*".repeat(word.len());
            // Perform case-insensitive replacement
            filtered = filtered.replace(word, &replacement);
        }
        filtered
    }
}

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
    filter: web::Data<Arc<ContentFilter>>,
) -> Result<HttpResponse, Error> {
    let mut user_info: Message = user.into_inner();

    // Filter the message content without converting to lowercase
    user_info.body = filter.filter_text(&user_info.body);

    let client: Client = db_pool.get().await.map_err(CustomErrors::PoolError)?;

    match db::add_message(&client, user_info).await {
        Ok(new_user) => {
            println!("Message added successfully: {:?}", new_user);
            Ok(HttpResponse::Ok().json(new_user))
        }
        Err(e) => {
            println!("Error adding message: {:?}", e);
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

    // Initialize content filter
    let filter = Arc::new(ContentFilter::new());

    let server = HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(filter.clone()))
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
