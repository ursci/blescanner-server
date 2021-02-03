CREATE TABLE device_logs (
  id          SERIAL NOT NULL PRIMARY KEY,
  gateway_id  INTEGER NOT NULL,
  device_uuid INTEGER NOT NULL,
  device_name VARCHAR NOT NULL,
  rssi        INTEGER NOT NULL,
  created_at  TIMESTAMP NOT NULL
);
