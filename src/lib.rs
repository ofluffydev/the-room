pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use models::{Message, NewMessage};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn send_message(
    conn: &mut PgConnection,
    username: &str,
    body: &str,
) -> Result<Message, diesel::result::Error> {
    use crate::schema::messages;

    let new_message = NewMessage { username, body };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .returning(Message::as_returning())
        .get_result(conn)
}
