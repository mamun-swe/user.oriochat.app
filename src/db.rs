use crate::models::{User, UserSingle};
use sqlx::mysql::MySqlPool;

pub async fn get_item(pool: &MySqlPool, id: i32) -> Option<UserSingle> {
    sqlx::query_as::<_, UserSingle>("SELECT id, username, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .expect("Failed to fetch item")
}

pub async fn get_item_by_email(pool: &MySqlPool, email: String) -> Option<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(pool)
        .await
        .expect("Failed to fetch item")
}

pub async fn create_item(
    pool: &MySqlPool,
    username: String,
    name: String,
    email: String,
    password: String,
) -> Result<UserSingle, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO users (username, name, email, password) VALUES (?, ?, ?, ?)",
        username,
        name,
        email,
        password
    )
    .execute(pool)
    .await?;

    let last_id = result.last_insert_id();
    let inserted_item = sqlx::query_as!(
        UserSingle,
        "SELECT id, username, name, email FROM users WHERE id = ?",
        last_id as i32
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_item)
}

pub async fn update_item(
    pool: &MySqlPool,
    id: i32,
    name: String,
    email: String,
) -> Result<UserSingle, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE users SET name = ?, email = ? WHERE id = ?",
        name,
        email,
        id
    )
    .execute(pool)
    .await?;

    let last_id = result.last_insert_id();
    let updated_item = sqlx::query_as!(
        UserSingle,
        "SELECT id, username, name, email FROM users WHERE id = ?",
        last_id as i32
    )
    .fetch_one(pool)
    .await?;

    Ok(updated_item)
}
