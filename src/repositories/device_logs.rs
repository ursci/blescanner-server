use actix_web::{web, Error, HttpResponse};
use async_trait::async_trait;

use crate::db::schema::device_logs::dsl::*;
use crate::diesel::{RunQueryDsl};
use crate::models::device_logs::DeviceLog;
use crate::Pool;

#[async_trait]
pub trait DeviceLogRepository: Send + Sync {
    async fn execute(
        &self,
        pool: web::Data<Pool>
    ) -> Result<HttpResponse, Error> {
        Ok(web::block(move || get_device_logs(pool))
          .await
          .map(|user| HttpResponse::Created().json(user))
          .map_err(|_| HttpResponse::InternalServerError())?,
        )
    }
}

fn get_device_logs(pool: web::Data<Pool>) -> Result<Vec<DeviceLog>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = device_logs.load::<DeviceLog>(&conn)?;
    Ok(items)
}
pub trait UsesDeviceLogRepository {
  type DeviceLogRepository: DeviceLogRepository;

  fn provide_device_log_repository(&self) -> &Self::DeviceLogRepository;
}
