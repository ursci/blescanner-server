table! {
    device_logs (id) {
        id -> Int4,
        gateway_name -> Varchar,
        location -> Varchar,
        device_uuid -> Varchar,
        device_name -> Nullable<Varchar>,
        rssi -> Int4,
        scanned_time -> Timestamp,
    }
}
