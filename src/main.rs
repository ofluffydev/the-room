use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use log::{error, info};
use the_room::models::{Message, NewMessage};

mod message_manager;

const INFO_MESSAGE: &str =
    "Welcome to the server! You can send both GET and POST requests to /messages.";

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(INFO_MESSAGE)
}

#[get("/messages")]
async fn get_messages() -> impl Responder {
    // Return literally all the messages on the Postgres database using diesel
    let messages: Vec<Message> = message_manager::grab_all_messages();

    if messages.is_empty() {
        return HttpResponse::Ok().body("No messages");
    }

    // Build a JSON string from the messages
    let message_string = match serde_json::to_string(&messages) {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().body("Error serializing messages"),
    };

    HttpResponse::Ok().body(message_string)
}

#[post("/messages")]
async fn send_message(req_body: String) -> impl Responder {
    /*
    {
        "username": "John Doe",
        "body": "Hello, world!"}
     */
    let message: NewMessage = match serde_json::from_str(&req_body) {
        Ok(msg) => msg,
        Err(_) => return HttpResponse::BadRequest().body("Invalid JSON format"),
    };

    info!("{} said: \"{}\"", message.username, message.body);

    // Add the message to the Postgres database using diesel
    let res = message_manager::add_message(&message.username, &message.body);

    HttpResponse::Ok().body(match res {
        Ok(_) => "Message sent successfully!",
        Err(e) => {
            error!("Error sending message: {}", e);
            "Error sending message!"
        }
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Starting up server...");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_messages)
            .service(send_message)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
