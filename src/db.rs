use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::CustomErrors, models::Message};

pub async fn get_messages(client: &Client) -> Result<Vec<Message>, CustomErrors> {
    let stmt = include_str!("../sql/get_messages.sql");
    let stmt = stmt.replace("$table_fields", &Message::sql_table_fields());
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
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &message_info.username,
                &message_info.body,
            ],
        )
        .await?
        .iter()
        .map(|row| Message::from_row_ref(row).unwrap())
        .collect::<Vec<Message>>()
        .pop()
        .ok_or(CustomErrors::NotFound) // more applicable for SELECTs
}