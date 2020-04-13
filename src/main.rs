mod config;
mod models;
mod handlers;
mod db;

use actix_web::{HttpServer, App, web};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::handlers::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Load env 
    dotenv().ok();
    let config=crate::config::Config::from_env().unwrap();
    println!("Starting Server on http://{}:{}", config.server.url, config.server.port);
    
    // Create postgres pool
    let pool=config.pg.create_pool(NoTls).unwrap();

    // Start Server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
    })
    .bind(format!("{}:{}", config.server.url, config.server.port))?
    .run()
    .await
}