#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::handlers::device_logs::UsesDeviceLogHandler;

mod models;
mod repositories;
mod usecases;
mod handlers;
mod db;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");


    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::resource("/device_logs")
                            .route(web::get().to(device_log_handler::<UsesDeviceLogUseCase>))
                    )
            )
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
