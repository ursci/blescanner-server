table! {
    device_logs (id) {
        id -> Int4,
        gateway_id -> Int4,
        device_uuid -> Nullable<Int4>,
        device_name -> Nullable<Varchar>,
        rssi -> Nullable<Float4>,
        created_at -> Timestamp,
    }
}
