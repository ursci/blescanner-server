table! {
    device_logs (id) {
        id -> Int4,
        gateway_id -> Int4,
        device_uuid -> Int4,
        device_name -> Varchar,
        rssi -> Int4,
        created_at -> Timestamp,
    }
}
