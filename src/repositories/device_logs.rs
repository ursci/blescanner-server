use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use chrono::Local;
use diesel::dsl::insert_into;

use crate::db::config::Pool;
use crate::db::schema::device_logs::dsl::*;
use crate::diesel::RunQueryDsl;
use crate::models::device_logs::DeviceLogResponse;
use crate::models::device_logs::{DeviceLogPayload, DeviceLogSchema};

/// Get device_logs
pub async fn get_device_logs(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get(db))
        .await
        .map(|logs| HttpResponse::Ok().json(logs))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

/// Post device_logs
pub async fn post_device_logs(
    db: web::Data<Pool>,
    data: web::Json<DeviceLogPayload>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || post(db, data))
        .await
        .map(|logs| HttpResponse::Ok().json(logs))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

/// Handle db connection and load the device logs from database.
fn get(db: web::Data<Pool>) -> Result<Vec<DeviceLogResponse>, diesel::result::Error> {
    let conn = db.get().unwrap();
    let items = device_logs.load::<DeviceLogResponse>(&conn)?;
    Ok(items)
}

/// Handle db connection and insert the device logs into database.
fn post(
    db: web::Data<Pool>,
    data: web::Json<DeviceLogPayload>,
) -> Result<DeviceLogResponse, diesel::result::Error> {
    let conn = db.get().unwrap();
    let device_log = DeviceLogSchema {
        gateway_id: &data.gateway_id,
        device_uuid: &data.device_uuid,
        device_name: &data.device_name,
        rssi: &data.rssi,
        created_at: Local::now().naive_local(),
    };
    let items = insert_into(device_logs)
        .values(&device_log)
        .get_result(&conn)?;
    Ok(items)
}
