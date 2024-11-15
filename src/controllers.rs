use crate::db::{create_item, get_item, get_item_by_email, update_item};
use crate::models::{LoginUser, User, UserCreate, UserSingle};
use actix_web::{web, HttpRequest, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Local;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    sub: String,
    name: String,
    username: String,
    exp: usize,
}

// Login a specific user by email and password
pub async fn login(pool: web::Data<MySqlPool>, item: web::Json<LoginUser>) -> HttpResponse {
    let user = get_item_by_email(&pool, item.email.clone()).await;
    if user.is_none() {
        return HttpResponse::NotFound().json("Invalid credentials.");
    }
    let user = user.unwrap();
    if bcrypt::verify(item.password.clone(), &user.password).unwrap() {
        let token = generate_token(user).await;
        return HttpResponse::Ok().json(token);
    }
    HttpResponse::Unauthorized().json("Invalid credentials.")
}

// Register a new user
pub async fn register(pool: web::Data<MySqlPool>, item: web::Json<UserCreate>) -> HttpResponse {
    // Generate a custom username
    let custom_username = format!("{}-{}", item.name.clone(), Local::now().timestamp_millis())
        .to_lowercase()
        .replace(" ", "-");

    // Hash the password without error handling
    let hash_password = hash(item.password.clone(), DEFAULT_COST).unwrap();

    // Check if the user already exists with the same email
    let existing_user = get_item_by_email(&pool, item.email.clone()).await;
    if existing_user.is_some() {
        return HttpResponse::Conflict().json("User already exists with the same email");
    }

    // Create the user with the custom username and hashed password
    let _ = create_item(
        &pool,
        custom_username,
        item.name.clone(),
        item.email.clone(),
        hash_password,
    )
    .await
    .unwrap();
    HttpResponse::Ok().json("Successfully account created")
}

// Get logged-in user profile
pub async fn get_profile(pool: web::Data<MySqlPool>, req: HttpRequest) -> HttpResponse {
    let id = req.headers().get("id").unwrap().to_str().unwrap();

    let item = get_item(&pool, id.parse().unwrap()).await;
    HttpResponse::Ok().json(item)
}

// Update logged-in user profile
pub async fn update_profile(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    item: web::Json<UserSingle>,
) -> HttpResponse {
    let id = req.headers().get("id").unwrap().to_str().unwrap();
    let _name = req.headers().get("name").unwrap().to_str().unwrap();
    let _username = req.headers().get("username").unwrap().to_str().unwrap();

    // Check email uniqueness
    let existing_user = get_item_by_email(&pool, item.email.clone()).await;
    if existing_user.is_some() && existing_user.unwrap().id.to_string() != id {
        return HttpResponse::Conflict().json("Another user already exists with the same email");
    }

    let _ = update_item(
        &pool,
        id.parse().unwrap(),
        item.name.clone(),
        item.email.clone(),
    )
    .await;
    HttpResponse::Ok().json("Profile successfully updated")
}

// Generate a JWT token for a specific user ID
async fn generate_token(user: User) -> String {
    let claims = JWTClaims {
        sub: user.id.to_string(),
        name: user.name,
        username: user.username,
        exp: (Local::now() + chrono::Duration::days(1)).timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("JWT_SECRET").expect("JWT_SECRET not set").as_ref()),
    )
    .unwrap()
}
