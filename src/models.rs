use super::schema::*;
use bigdecimal::BigDecimal;

use diesel::Identifiable;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    pub status: u16,
    pub data: T,
}

#[derive(DbEnum, Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name="users"]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub barcode: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub crated_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable)]
#[primary_key(transaction_id, user_id)]
pub struct TransactionProduct {
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub full_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub role: Option<Role>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "products"]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub barcode: Option<&'a str>,
    pub price: BigDecimal,
    pub stock: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct InputLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct InputUser {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub role: Option<Role>
}

#[derive(Deserialize)]
pub struct InputProduct {
    pub name: String,
    pub barcode: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
}