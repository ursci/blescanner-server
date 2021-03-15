import { DeviceLog } from "../models/device_logs";
import { DeviceLogJSON } from "../entities/device_logs";

export class DeviceLogRepository {
  //
  private readonly deviceLogEndpoint = `/device_logs`;

  public async fetchDeviceLogs(): Promise<Array<DeviceLog>> {
    const requestURL = `${this.deviceLogEndpoint}`;
    const rawResponse = await fetch(requestURL);
    const result: DeviceLogJSON = await rawResponse.json();
    const deviceLog: Array<DeviceLog> = await result.data;
    return deviceLog;
  }
}
