use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

use crate::db::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct DeviceLog {
    pub id: i32,
    pub gateway_id: i32,
    pub device_uuid: Option<i32>,
    pub device_name: Option<String>,
    pub rssi: Option<f32>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "device_logs"]
pub struct DeviceLogSchema {
    pub gateway_id: i32,
    pub device_uuid: Option<i32>,
    pub device_name: Option<String>,
    pub rssi: Option<f32>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceLogRequest {
    pub id: u32,
    pub gateway_id: u32,
    pub device_uuid: u32,
    pub device_name: String,
    // pub rssi: Option<f32>,
    // pub created_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeviceLogResponse {
    pub id: Option<u32>,
    pub gateway_id: u32,
    pub device_uuid: u32,
    pub device_name: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostDeviceLogResponse {
    pub id: Option<u32>,
    pub gateway_id: u32,
    pub device_name: Option<String>,
    pub created_at: NaiveDateTime,
}
