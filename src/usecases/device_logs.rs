use async_trait::async_trait;

use crate::repositories::device_logs::UsesDeviceLogRepository;

#[async_trait]
pub trait DeviceLogUseCase: UsesDeviceLogRepository + Sync {
    async fn get_device_logs(&self) {
      self.provide_device_log_repository();
    }
}

impl<T: UsesDeviceLogRepository + Sync> DeviceLogUseCase for T {}

pub trait UsesDeviceLogUseCase {
    type DeviceLogUseCase: DeviceLogUseCase;

    fn provide_device_log_usecase(&self) -> Self::DeviceLogUseCase;
}
