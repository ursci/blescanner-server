use actix_web::{web, HttpResponse};
use anyhow::Result;
use async_trait::async_trait;

use crate::errors::BleScannerApiError;
use crate::models::device_logs::DeviceLogs;
use crate::repositories::device_logs::{
    HaveDeviceLogRepository, IsDeviceLogRepository, MockDeviceLogRepository,
};

#[async_trait(?Send)]
pub trait IsDeviceLogUseCase: HaveDeviceLogRepository {
    async fn get_device_logs(&self) -> Result<HttpResponse, BleScannerApiError> {
        let repository = self.provide_device_logs_repository();
        let result = repository.get_device_logs().await;
        result
    }

    async fn post_device_logs(
        &self,
        payloads: web::Json<DeviceLogs>,
    ) -> Result<HttpResponse, BleScannerApiError> {
        let repository = self.provide_device_logs_repository();
        let result = repository.post_device_logs(payloads).await;
        result
    }
}

pub trait HaveDeviceLogUseCase {
    type DeviceLogUseCase: IsDeviceLogUseCase;

    fn provide_devicelog_usecase(&self) -> &Self::DeviceLogUseCase;
}

impl<T: HaveDeviceLogRepository> IsDeviceLogUseCase for T {}

impl HaveDeviceLogUseCase for MockDeviceLogRepository {
    type DeviceLogUseCase = MockDeviceLogRepository;

    fn provide_devicelog_usecase(&self) -> &Self::DeviceLogUseCase {
        &self
    }
}
