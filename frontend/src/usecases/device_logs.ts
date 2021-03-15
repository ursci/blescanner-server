import { DeviceLogRepository } from "../repositories/device_logs";
import { DeviceLog } from "../models/device_logs";

export class DeviceLogUseCase {
  constructor(private readonly deviceLogRepository: DeviceLogRepository) {}

  public async fetchDeviceLogs(): Promise<Array<DeviceLog>> {
    const deviceLogs = await this.deviceLogRepository.fetchDeviceLogs();
    return deviceLogs;
  }
}
