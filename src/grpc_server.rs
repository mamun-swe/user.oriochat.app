use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use tonic::{transport::Server, Request, Response, Status};

pub mod token_service {
    tonic::include_proto!("token");
}

use token_service::token_service_server::{TokenService, TokenServiceServer};
use token_service::{TokenRequest, TokenResponse};

#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    sub: String,
    name: String,
    username: String,
    exp: usize,
}

#[derive(Default)]
pub struct MyTokenService;

#[tonic::async_trait]
impl TokenService for MyTokenService {
    async fn get_token_info(
        &self,
        request: Request<TokenRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        let token = request.into_inner().token;

        // Replace with your JWT secret
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET not set")
            .to_string();

        let validation = Validation::new(Algorithm::HS256);
        match decode::<JWTClaims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &validation,
        ) {
            Ok(token_data) => {
                let claims = token_data.claims;

                let response = TokenResponse {
                    id: claims.sub,
                    name: claims.name,
                    username: claims.username,
                    error: "".to_string(),
                };

                println!("I am here from gRPC request.");

                Ok(Response::new(response))
            }
            Err(err) => {
                let response = TokenResponse {
                    id: "".to_string(),
                    name: "".to_string(),
                    username: "".to_string(),
                    error: format!("Failed to parse token: {}", err),
                };

                Ok(Response::new(response))
            }
        }
    }
}

pub async fn run_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let user_service = MyTokenService::default();

    println!("Starting gRPC server at {:?}", addr);

    Server::builder()
        .add_service(TokenServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
