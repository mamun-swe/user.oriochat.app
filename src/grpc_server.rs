use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tonic::{transport::Server, Request, Response, Status};
use std::env;

pub mod user_service {
    tonic::include_proto!("user");
}

use user_service::user_service_server::{UserService, UserServiceServer};
use user_service::{UserRequest, UserResponse};

#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    sub: String,
    name: String,
    username: String,
    exp: usize,
}

#[derive(Default)]
pub struct MyUserService;

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn get_user_info(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let token = request.into_inner().jwt_token;

        // Replace with your JWT secret
        let jwt_secret = env::var("JWT_SECRET")
                .expect("JWT_SECRET not set")
                .to_string();

        let validation = Validation::new(Algorithm::HS256);
        match decode::<JWTClaims>(&token, &DecodingKey::from_secret(jwt_secret.as_ref()), &validation) {
            Ok(token_data) => {
                let claims = token_data.claims;

                let response = UserResponse {
                    id: claims.sub,
                    name: claims.name,
                    username: claims.username,
                    error: "".to_string(),
                };

                Ok(Response::new(response))
            }
            Err(err) => {
                let response = UserResponse {
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
    let addr = "[::1]:50051".parse()?;
    let user_service = MyUserService::default();

    println!("Starting gRPC server at {:?}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
