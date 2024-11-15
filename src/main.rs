use actix_web::{middleware::from_fn, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
mod middleware;
use crate::middleware::my_middleware;

mod controllers;
mod db;
mod models;
mod grpc_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    tokio::spawn(async {
        if let Err(e) = grpc_server::run_grpc_server().await {
            eprintln!("Error running gRPC server: {}", e);
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/profile")
                    .wrap(from_fn(my_middleware))
                    .route("", web::get().to(controllers::get_profile))
                    .route("", web::put().to(controllers::update_profile)),
            )
            .route("/login", web::post().to(controllers::login))
            .route("/register", web::post().to(controllers::register))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
