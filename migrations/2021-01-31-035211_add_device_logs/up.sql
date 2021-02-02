CREATE TABLE device_logs (
  id          SERIAL NOT NULL PRIMARY KEY,
  gateway_id  INTEGER NOT NULL,
  device_uuid INTEGER,
  device_name VARCHAR,
  rssi        REAL,
  created_at  TIMESTAMP NOT NULL
);
