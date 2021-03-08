use actix_web::{web, HttpResponse};
use anyhow::Result;

use crate::{
    errors::BleScannerApiError,
    models::device_logs::DeviceLogs,
    repositories::device_logs::MockDeviceLogRepository,
    usecases::device_logs::{HaveDeviceLogUseCase, IsDeviceLogUseCase},
};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::device_logs::{DeviceLogPayload, DeviceLogs};
    use actix_web::{http, web::Json};
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    #[actix_rt::test]
    async fn test_get_devicelog_handler() {
        let test_mock = MockDeviceLogRepository {};
        let response = get_device_logs(&test_mock).await;
        let result = response.unwrap();

        assert_eq!(result.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_post_devicelog_handler() {
        let test_mock = MockDeviceLogRepository {};

        let d = NaiveDate::from_ymd(2021, 3, 1);
        let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
        let dt = NaiveDateTime::new(d, t);

        let payload_mock = DeviceLogPayload {
            device_name: "".to_string(),
            location_name: "".to_string(),
            scanned_name: Some("".to_string()),
            scanned_id: "".to_string(),
            scanned_rssi: 10000,
            scanned_time: dt,
        };

        let mock_logs = DeviceLogs {
            device_logs: vec![payload_mock],
        };

        let response = post_device_logs(&test_mock, Json(mock_logs)).await;
        let result = response.unwrap();

        assert_eq!(result.status(), http::StatusCode::OK);
    }
}
