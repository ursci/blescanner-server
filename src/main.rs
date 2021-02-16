#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use actix_web::{middleware, web, App, HttpServer};

use crate::db::config::establish_connection;
use crate::handlers::device_logs::{get_device_logs, post_device_logs};

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
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1").service(
                    web::resource("/device_logs")
                        .route(web::get().to(get_device_logs))
                        .route(web::post().to(post_device_logs)),
                ),
            )
    })
    .bind(&url)?
    .run()
    .await
}
