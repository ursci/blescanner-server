use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::db::schema::*;

/// Data type that sent from the gateway to `POST /api/v1/device_logs`
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceLogPayload {
    pub gateway_id: i32,
    pub device_uuid: i32,
    pub device_name: String,
    pub rssi: i32,
}

/// Data type for the `device_logs` API response.
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct DeviceLogResponse {
    pub id: i32,
    pub gateway_id: i32,
    pub device_uuid: i32,
    pub device_name: String,
    pub rssi: i32,
    pub created_at: NaiveDateTime,
}

/// Data type for `device_logs` table
#[derive(Insertable, Debug)]
#[table_name = "device_logs"]
pub struct DeviceLogSchema<'a> {
    pub gateway_id: &'a i32,
    pub device_uuid: &'a i32,
    pub device_name: &'a String,
    pub rssi: &'a i32,
    pub created_at: NaiveDateTime,
}
