use actix_web::{web, HttpResponse};
use anyhow::Result;
use async_trait::async_trait;

use crate::errors::BleScannerApiError;
use crate::models::device_logs::DeviceLogs;
use crate::repositories::device_logs::{IsDeviceLogRepository, HaveDeviceLogRepository};
use crate::{db::config::Pool, repositories::device_logs::DbContext};

#[async_trait(?Send)]
pub trait IsDeviceLogUseCase: HaveDeviceLogRepository {
    async fn get_device_logs(
        &self,
        db: web::Data<Pool>,
    ) -> Result<HttpResponse, BleScannerApiError> {
        let repository = self.provide_device_logs_repository();
        let result = repository.get_device_logs(db).await;
        result
    }

    async fn post_device_logs(
        &self,
        db: web::Data<Pool>,
        payloads: web::Json<DeviceLogs>,
    ) -> Result<HttpResponse, BleScannerApiError> {
        let repository = self.provide_device_logs_repository();
        let result = repository.post_device_logs(db, payloads).await;
        result
    }
}

pub trait HaveDeviceLogUseCase {
    type DeviceLogUseCase: IsDeviceLogUseCase;

    fn provide_devicelog_usecase(&self) -> &Self::DeviceLogUseCase;
}

impl<T: HaveDeviceLogRepository> IsDeviceLogUseCase for T {}

impl HaveDeviceLogUseCase for DbContext {
    type DeviceLogUseCase = DbContext;

    fn provide_devicelog_usecase(&self) -> &Self::DeviceLogUseCase {
        &self
    }
}
