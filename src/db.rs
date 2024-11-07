use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::CustomErrors, models::Message};

pub async fn get_messages(
    client: &Client,
    last: Option<usize>,
) -> Result<Vec<Message>, CustomErrors> {
    let mut stmt = include_str!("../sql/get_messages.sql").to_string();
    stmt = stmt.replace("$table_fields", &Message::sql_table_fields());

    if let Some(count) = last {
        stmt.push_str(" ORDER BY id DESC");
        stmt.push_str(&format!(" LIMIT {}", count));
    }

    // Push the ";"
    stmt.push_str(";");

    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Message::from_row_ref(row).unwrap())
        .collect::<Vec<Message>>();

    Ok(results)
}

pub async fn add_message(client: &Client, message_info: Message) -> Result<Message, CustomErrors> {
    let _stmt = include_str!("../sql/send_message.sql");
    let _stmt = _stmt.replace("$table_fields", &Message::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(|_| CustomErrors::DatabaseError)?;

    let messages = client
        .query(&stmt, &[&message_info.username, &message_info.body])
        .await
        .map_err(|_| CustomErrors::DatabaseError)?
        .iter()
        .map(|row| Message::from_row_ref(row).unwrap())
        .collect::<Vec<Message>>();

    messages.into_iter().next().ok_or(CustomErrors::NotFound) // Only return NotFound if no message is found
}
