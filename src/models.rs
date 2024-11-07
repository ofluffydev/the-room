use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "messages")] // singular 'user' is a keyword..
pub struct Message {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub body: String,
}
