use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserCreate {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserSingle {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
