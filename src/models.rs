#[derive(Queryable)]
pub struct User {
    id: String,
    full_name: String,
    email: String,
    password: String
}