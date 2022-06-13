use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password: String
}

