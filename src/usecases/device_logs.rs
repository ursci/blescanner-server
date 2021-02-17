use actix_web::{web, HttpResponse};
use anyhow::Result;

use crate::db::config::Pool;
use crate::errors::BleScannerError;
use crate::models::device_logs::DeviceLogs;
use crate::repositories::device_logs::get_device_logs as list;
use crate::repositories::device_logs::post_device_logs as post;

pub async fn get_device_logs(db: web::Data<Pool>) -> Result<HttpResponse, BleScannerError> {
    list(db).await
}

pub async fn post_device_logs(
    db: web::Data<Pool>,
    payloads: web::Json<DeviceLogs>,
) -> Result<HttpResponse, BleScannerError> {
    post(db, payloads).await
}
