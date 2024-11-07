use crate::schema::messages;
use diesel::prelude::*;

#[derive(Queryable, Selectable, QueryableByName, serde::Deserialize, serde::Serialize)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i32,
    pub username: String,
    pub body: String,
    pub time: chrono::NaiveDateTime,
}

#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub username: &'a str,
    pub body: &'a str,
}
