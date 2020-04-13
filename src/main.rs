mod config;
mod models;

use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder};
use std::io;
use dotenv::dotenv;

async fn status() -> impl Responder{
    web::HttpResponse::Ok()
        .json(Status {status: "OK".to_string()})
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Load env 
    dotenv().ok();
    let config=crate::config::Config::from_env().unwrap();
    println!("Starting Server on http://{}:{}", config.server.url, config.server.port);
    
    // Start Server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}