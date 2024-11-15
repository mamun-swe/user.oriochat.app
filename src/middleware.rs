use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error,
    http::header::{HeaderName, HeaderValue},
    middleware::Next,
    Error,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    sub: String,
    name: String,
    username: String,
    exp: usize,
}

pub struct JWTKey {
    pub secret: String,
}

impl JWTKey {
    pub fn verify_token<T>(&self, token: &str) -> Result<T, jsonwebtoken::errors::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let decoding_key = DecodingKey::from_secret(self.secret.as_ref());
        let validation = Validation::new(Algorithm::HS256);
        decode::<T>(token, &decoding_key, &validation).map(|data| data.claims)
    }
}

pub async fn my_middleware(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        let token = auth_header
            .to_str()
            .unwrap_or("")
            .trim_start_matches("Bearer ")
            .to_string();

        if token.is_empty() {
            return Err(error::ErrorNotFound("Authorization token is missing"));
        }

        let key = JWTKey {
            secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET not set")
                .to_string(),
        };

        let claims: JWTClaims = match key.verify_token(&token) {
            Ok(claims) => claims,
            Err(_err) => {
                return Err(error::ErrorNotFound(format!("Invalid authorization token")));
            }
        };

        // Extract user ID, name, and username from claims
        let user_id = claims.sub.clone();
        let name = claims.name.clone();
        let username = claims.username.clone();

        // Set custom headers in the request
        req.headers_mut().insert(
            HeaderName::from_static("id"),
            HeaderValue::from_str(&user_id).unwrap(),
        );
        req.headers_mut().insert(
            HeaderName::from_static("name"),
            HeaderValue::from_str(&name).unwrap(),
        );
        req.headers_mut().insert(
            HeaderName::from_static("username"),
            HeaderValue::from_str(&username).unwrap(),
        );
    } else {
        return Err(error::ErrorNotFound("Authorization header is missing"));
    }

    next.call(req).await
}
