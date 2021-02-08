CREATE TABLE device_logs (
  id            SERIAL NOT NULL ,
  gateway_name  VARCHAR NOT NULL,
  location      VARCHAR NOT NULL,
  device_uuid   VARCHAR NOT NULL,
  device_name   VARCHAR,
  rssi          INTEGER NOT NULL,
  scanned_time  TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  PRIMARY KEY (gateway_name, device_uuid, scanned_time)
);
