use actix_web::{web, HttpResponse};
use anyhow::Result;

use crate::db::config::Pool;
use crate::errors::BleScannerApiError;
use crate::models::device_logs::DeviceLogs;
use crate::repositories::device_logs::DbContext;
use crate::usecases::device_logs::{HaveDeviceLogUseCase, IsDeviceLogUseCase};

/// Handler to GET /api/v1/device_logs
pub async fn get_devicelog_handler(
    db: web::Data<Pool>,
) -> Result<HttpResponse, BleScannerApiError> {
    let context = DbContext {};
    let result = get_device_logs(&context, db).await;
    result
}

/// Handler to POST /api/v1/device_logs
pub async fn post_devicelog_handler(
    db: web::Data<Pool>,
    payloads: web::Json<DeviceLogs>,
) -> Result<HttpResponse, BleScannerApiError> {
    let context = DbContext {};
    let result = post_device_logs(&context, db, payloads).await;
    result
}

async fn get_device_logs<T>(
    context: &T,
    db: web::Data<Pool>,
) -> Result<HttpResponse, BleScannerApiError>
where
    T: HaveDeviceLogUseCase,
{
    let usecase = context.provide_devicelog_usecase();
    let result = usecase.get_device_logs(db).await;
    result
}

async fn post_device_logs<T>(
    context: &T,
    db: web::Data<Pool>,
    payloads: web::Json<DeviceLogs>,
) -> Result<HttpResponse, BleScannerApiError>
where
    T: HaveDeviceLogUseCase,
{
    let usecase = context.provide_devicelog_usecase();
    let result = usecase.post_device_logs(db, payloads).await;
    result
}
