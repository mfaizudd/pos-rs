use serde::Serialize;
use uuid::Uuid;
use super::schema::users;

#[derive(Queryable)]
#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub full_name: &'a str,
    pub email: &'a str,
    pub password: &'a str
}
