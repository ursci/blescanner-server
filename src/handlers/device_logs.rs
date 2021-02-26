use actix_web::{web, HttpResponse};
use anyhow::Result;

use crate::errors::BleScannerApiError;
use crate::models::device_logs::DeviceLogs;
use crate::repositories::device_logs::MockDeviceLogRepository;
use crate::usecases::device_logs::{HaveDeviceLogUseCase, IsDeviceLogUseCase};

/// Handler to GET /api/v1/device_logs
pub async fn get_devicelog_handler() -> Result<HttpResponse, BleScannerApiError> {
    let context = MockDeviceLogRepository {};
    let result = get_device_logs(&context).await;
    result
}

/// Handler to POST /api/v1/device_logs
pub async fn post_devicelog_handler(
    payloads: web::Json<DeviceLogs>,
) -> Result<HttpResponse, BleScannerApiError> {
    let context = MockDeviceLogRepository {};
    let result = post_device_logs(&context, payloads).await;
    result
}

async fn get_device_logs<T>(context: &T) -> Result<HttpResponse, BleScannerApiError>
where
    T: HaveDeviceLogUseCase,
{
    let usecase = context.provide_devicelog_usecase();
    let result = usecase.get_device_logs().await;
    result
}

async fn post_device_logs<T>(
    context: &T,
    payloads: web::Json<DeviceLogs>,
) -> Result<HttpResponse, BleScannerApiError>
where
    T: HaveDeviceLogUseCase,
{
    let usecase = context.provide_devicelog_usecase();
    let result = usecase.post_device_logs(payloads).await;
    result
}
