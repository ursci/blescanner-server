import { DateTime } from "luxon";

export type DeviceLog = {
  id: number;
  gateway_name: string;
  location: string;
  device_uuid: string;
  device_name?: string;
  rssi: number;
  scanned_time: DateTime;
};
