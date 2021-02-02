/*
 * device_logs
 */
CREATE TABLE IF NOT EXISTS  device_logs (
    id          INTEGER NOT NULL PRIMARY KEY,
    gateway_id  INTEGER NOT NULL,
    device_uuid INTEGER,
    device_name TEXT,
    rssi        REAL,
    created_at  TEXT,
    updated_at  TEXT,
    deleted_at  TEXT,
    UNIQUE(device_uuid)
);
