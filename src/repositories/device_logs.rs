use actix_web::{web, HttpResponse};
use anyhow::Result;
use diesel::dsl::insert_into;

use crate::db::schema::device_logs::dsl::*;
use crate::diesel::RunQueryDsl;
use crate::errors::BleScannerError;
use crate::models::device_logs::{
    DeviceLogSchema, DeviceLogs, GetDeviceLogResponse, PostDeviceLogResponse,
};
use crate::{db::config::Pool, models::device_logs::DeviceLogQuery};

use super::{HTTP_STATUS, REQUEST_SUCCEEDED, RESOURCE_CREATED};

/// Get device_logs
pub async fn get_device_logs(db: web::Data<Pool>) -> Result<HttpResponse, BleScannerError> {
    Ok(web::block(move || handle_load(db))
        .await
        .map(|logs| {
            HttpResponse::Ok()
                .header(HTTP_STATUS, REQUEST_SUCCEEDED)
                .json(logs)
        })
        .map_err(|_| BleScannerError::InternalError)?)
}

/// Post device_logs
pub async fn post_device_logs(
    db: web::Data<Pool>,
    payloads: web::Json<DeviceLogs>,
) -> Result<HttpResponse, BleScannerError> {
    Ok(web::block(move || handle_insert(db, payloads))
        .await
        .map(|logs| {
            HttpResponse::Ok()
                .header(HTTP_STATUS, RESOURCE_CREATED)
                .json(logs)
        })
        .map_err(|_| BleScannerError::InternalError)?)
}

/// Handle db connection and load the device logs from database.
fn handle_load(db: web::Data<Pool>) -> Result<GetDeviceLogResponse, diesel::result::Error> {
    let conn = db.get().unwrap();

    let queried_item: Vec<DeviceLogQuery> = device_logs.load::<DeviceLogQuery>(&conn)?;

    // Convert the data for API response type
    let result = GetDeviceLogResponse { data: queried_item };

    Ok(result)
}

/// Handle db connection and insert the device logs into database.
fn handle_insert(
    db: web::Data<Pool>,
    payloads: web::Json<DeviceLogs>,
) -> Result<PostDeviceLogResponse, diesel::result::Error> {
    let empty_device_name = "No device name".to_string();
    let conn = db.get().unwrap();

    // Convert the data for table schema
    let converted_logs = payloads
        .device_logs
        .iter()
        .map(|log| DeviceLogSchema {
            gateway_name: &log.device_name,
            location: &log.location_name,
            device_uuid: &log.scanned_id,
            device_name: &log.scanned_name.as_ref().unwrap_or(&empty_device_name),
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
