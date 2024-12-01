use actix_cors::Cors;
use actix_web::{middleware::from_fn, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
mod middleware;
use crate::middleware::my_middleware;
use std::time::Duration;
use tokio::time::sleep;

mod controllers;
mod db;
mod grpc_server;
mod models;

async fn connect_to_database(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    let mut retries = 0;
    loop {
        match MySqlPool::connect(database_url).await {
            Ok(pool) => return Ok(pool),
            Err(_) => {
                retries += 1;
                if retries > 5 {
                    return Err(sqlx::Error::PoolTimedOut); // Or handle this as per your needs
                }
                // Sleep for 5 seconds before retrying
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Attempt to connect to the database with retries
    let pool = match connect_to_database(&database_url).await {
        Ok(pool) => {
            println!("Successfully connected to the database");
            pool
        }
        Err(e) => {
            eprintln!(
                "Failed to connect to the database after several retries: {}",
                e
            );
            return Ok(()); // Return or exit the application if DB is not available
        }
    };

    tokio::spawn(async {
        if let Err(e) = grpc_server::run_grpc_server().await {
            eprintln!("Error running gRPC server: {}", e);
        }
    });

    // Log the address where the server will run
    let server_address = "0.0.0.0:5000";
    println!("Starting server at http://{}", server_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
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
    .bind(server_address)?
    .run()
    .await
}
