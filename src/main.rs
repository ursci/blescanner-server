#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use actix_web::{
    middleware,
    web::{self},
    App, HttpServer,
};

use crate::db::config::establish_connection;
use crate::handlers::device_logs::{get_devicelog_handler, post_devicelog_handler};

const JSON_SIZE_LIMIT: usize = 1024 * 1024 * 50;

mod db;
mod errors;
mod handlers;
mod models;
mod repositories;
mod usecases;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let host = dotenv::var("HOST").expect("HOST not found.");
    let port = dotenv::var("PORT").expect("PORT not found.");
    let url = format!("{}:{}", &host, &port);
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(web::JsonConfig::default().limit(JSON_SIZE_LIMIT))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1").service(
                    web::resource("/device_logs")
                        .route(web::get().to(get_devicelog_handler))
                        .route(web::post().to(post_devicelog_handler)),
                ),
            )
    })
    .bind(&url)?
    .run()
    .await
}
