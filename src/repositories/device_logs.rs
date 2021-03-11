use actix_web::{web, HttpResponse};
use anyhow::Result;
use async_trait::async_trait;
use diesel::dsl::insert_into;

use crate::db::config::establish_connection;
use crate::db::schema::device_logs::dsl::*;
use crate::diesel::RunQueryDsl;
use crate::errors::{BleScannerApiError, BleScnnerDbError};
use crate::models::device_logs::{
    DeviceLogQuery, DeviceLogSchema, DeviceLogs, GetDeviceLogResponse, PostDeviceLogResponse,
};

static HTTP_STATUS: &str = "Status";
static REQUEST_SUCCEEDED: &str = "200 OK";
static RESOURCE_CREATED: &str = "201 Created";

#[async_trait(?Send)]
pub trait IsDeviceLogRepository {
    async fn get_device_logs(&self) -> Result<HttpResponse, BleScannerApiError>;

    async fn post_device_logs(
        &self,
        payloads: web::Json<DeviceLogs>,
    ) -> Result<HttpResponse, BleScannerApiError>;
}

pub trait HaveDeviceLogRepository {
    type DeviceLogRepository: IsDeviceLogRepository;

    fn provide_device_logs_repository(&self) -> &Self::DeviceLogRepository;
}

#[derive(Clone, Debug)]
pub struct MockDeviceLogRepository {}

#[async_trait(?Send)]
impl IsDeviceLogRepository for MockDeviceLogRepository {
    /// Get device_logs
    #[rustfmt::skip]
    async fn get_device_logs(&self) -> Result<HttpResponse, BleScannerApiError> {
        Ok(web::block(move || {handle_load()})
            .await
            .map(|logs| {
                HttpResponse::Ok()
                    .header(HTTP_STATUS, REQUEST_SUCCEEDED)
                    .json(logs)
            })
            .map_err(|_| BleScannerApiError::InternalError)?)
    }

    /// Post device_logs
    async fn post_device_logs(
        &self,
        payloads: web::Json<DeviceLogs>,
    ) -> Result<HttpResponse, BleScannerApiError> {
        Ok(web::block(move || handle_insert(payloads))
            .await
            .map(|logs| {
                HttpResponse::Ok()
                    .header(HTTP_STATUS, RESOURCE_CREATED)
                    .json(logs)
            })
            .map_err(|_| BleScannerApiError::InternalError)?)
    }
}

impl HaveDeviceLogRepository for MockDeviceLogRepository {
    type DeviceLogRepository = MockDeviceLogRepository;

    fn provide_device_logs_repository(&self) -> &Self::DeviceLogRepository {
        &self
    }
}

/// Handle db connection and load the device logs from database.
fn handle_load() -> Result<GetDeviceLogResponse, BleScnnerDbError> {
    let pool = establish_connection();
    let conn = pool.get()?;

    let queried_item: Vec<DeviceLogQuery> = device_logs.load::<DeviceLogQuery>(&conn)?;

    // Convert the data for API response type
    let result = GetDeviceLogResponse { data: queried_item };
    Ok(result)
}

/// Handle db connection and insert the device logs into database.
fn handle_insert(
    payloads: web::Json<DeviceLogs>,
) -> Result<PostDeviceLogResponse, BleScnnerDbError> {
    let pool = establish_connection();
    let conn = pool.get()?;

    let empty_device_name = "No device name".to_string();
    // Convert the data for table schema
    let converted_logs = payloads
        .device_logs
        .iter()
        .map(|log| DeviceLogSchema {
            gateway_name: &log.scanned_name.as_ref().unwrap_or(&empty_device_name),
            location: &log.location_name,
            device_uuid: &log.scanned_id,
            device_name: &log.device_name,
            rssi: &log.scanned_rssi,
            scanned_time: &log.scanned_time,
        })
        .collect::<Vec<DeviceLogSchema>>();

    let queried_item = insert_into(device_logs)
        .values(converted_logs)
        .execute(&conn)?;

    // Convert the data for API response type
    let result = PostDeviceLogResponse { data: queried_item };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::config::establish_test_connection;
    use crate::models::device_logs::{DeviceLogPayload, DeviceLogs};
    use actix_web::web::Json;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use diesel::delete;

    #[test]
    fn test_handle_insert() {
        let d = NaiveDate::from_ymd(2021, 3, 3);
        let t = NaiveTime::from_hms_milli(12, 35, 56, 789);
        let dt = NaiveDateTime::new(d, t);

        let payload_test1 = DeviceLogPayload {
            device_name: "test_device".to_string(),
            location_name: "tokyo_museum".to_string(),
            scanned_name: Some("test_gateway".to_string()),
            scanned_id: "001".to_string(),
            scanned_rssi: 10000,
            scanned_time: dt,
        };

        let payload_test2 = DeviceLogPayload {
            device_name: "test_device2".to_string(),
            location_name: "tokyo_museum".to_string(),
            scanned_name: Some("test_gateway2".to_string()),
            scanned_id: "002".to_string(),
            scanned_rssi: 10001,
            scanned_time: dt,
        };

        let payloads = DeviceLogs {
            device_logs: vec![payload_test1, payload_test2],
        };

        let connection = establish_test_connection().unwrap();
        let _ = delete(device_logs).execute(&connection).unwrap();

        let insert_result = handle_insert(Json(payloads)).unwrap();
        assert_eq!(insert_result.data, 2);
    }
}
