use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

#[derive(Insertable, AsChangeset)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub full_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}
