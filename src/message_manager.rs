use the_room::*;
use self::models::*;
use diesel::prelude::*;



pub fn add_message(username: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = &mut establish_connection();
    send_message(conn, username, body)?;
    Ok(())
}

pub fn grab_all_messages() -> Vec<Message> {
    use schema::messages::dsl::*;

    let connection = &mut establish_connection();
    let results = messages
        .limit(5)
        .select(Message::as_select())
        .load(connection)
        .expect("Error loading posts");

    results
}

#[allow(dead_code)]
pub fn grab_last_messages(amount: i64) -> Vec<Message> {
    use schema::messages::dsl::*;

    let connection = &mut establish_connection();
    let results = messages
        .order(id.desc())
        .limit(amount)
        .select(Message::as_select())
        .load(connection)
        .expect("Error loading messages");

    results
}
