use actix_web::{web, HttpResponse};
use anyhow::Result;

use crate::db::config::Pool;
use crate::errors::BleScannerApiError;
use crate::models::device_logs::DeviceLogs;
use crate::usecases::device_logs::get_device_logs as list;
use crate::usecases::device_logs::post_device_logs as post;

/// Handler to GET /api/v1/device_logs
pub async fn get_device_logs(db: web::Data<Pool>) -> Result<HttpResponse, BleScannerApiError> {
    list(db).await
}

/// Handler to POST /api/v1/device_logs
pub async fn post_device_logs(
    db: web::Data<Pool>,
    payloads: web::Json<DeviceLogs>,
) -> Result<HttpResponse, BleScannerApiError> {
    post(db, payloads).await
}
