use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::db::schema::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceLogPayload {
    pub device_name: String,
    pub location_name: String,
    /// Older Bluetooth sensors will have no name
    pub scanned_name: Option<String>,
    pub scanned_id: String,
    pub scanned_rssi: i32,
    // Replaced from `created_at` because the scanned time is asynchronously send to the server.
    pub scanned_time: NaiveDateTime,
}

/// Data type that sent from the gateway to `POST /api/v1/device_logs`
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceLogs {
    pub device_logs: Vec<DeviceLogPayload>,
}

/// Data type that querying from `device_logs`.
#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct DeviceLogQuery {
    pub id: i32,
    pub gateway_name: String,
    pub location: String,
    pub device_uuid: String,
    pub device_name: Option<String>,
    pub rssi: i32,
    // Replaced from `created_at` because the scanned time is asynchronously send to the server.
    pub scanned_time: NaiveDateTime,
}

/// Data type for the `GET /api/v1/device_logs` API response.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeviceLogResponse {
    pub data: Vec<DeviceLogQuery>,
}

/// Data type for the `POST /api/v1/device_logs` API response.
#[derive(Debug, Serialize, Deserialize)]
pub struct PostDeviceLogResponse {
    pub data: usize,
}

/// Data type for `device_logs` table
#[derive(Insertable, Debug)]
#[table_name = "device_logs"]
pub struct DeviceLogSchema<'a> {
    pub gateway_name: &'a str,
    pub location: &'a str,
    pub device_uuid: &'a str,
    pub device_name: &'a str,
    pub rssi: &'a i32,
    // Replaced from `created_at` because the scanned time is asynchronously send to the server.
    pub scanned_time: &'a NaiveDateTime,
}
