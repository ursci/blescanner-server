use actix_web::{web, Error, HttpResponse};
use anyhow::Result;

use crate::db::config::Pool;
use crate::models::device_logs::DeviceLogPayload;
use crate::repositories::device_logs::get_device_logs as list;
use crate::repositories::device_logs::post_device_logs as post;

pub async fn get_device_logs(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    list(db).await
}

pub async fn post_device_logs(db: web::Data<Pool>, data: web::Json<DeviceLogPayload>,) -> Result<HttpResponse, Error> {
    post(db, data).await
}
