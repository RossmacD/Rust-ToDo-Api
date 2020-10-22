mod config;
mod db;
mod handlers;
mod models;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

use crate::handlers::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Load env
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    println!(
        "Starting Server on http://{}:{}",
        config.server.url, config.server.port
    );
    // Create postgres pool
    let pool = config.pg.create_pool(NoTls).unwrap();

    // Initialise debug logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Start Server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/items/items/{list_id}{_:/?}", web::get().to(get_items))
    })
    .bind(format!("{}:{}", config.server.url, config.server.port))?
    .run()
    .await
}
