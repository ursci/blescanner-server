use actix_web::{HttpResponse, web};
use async_trait::async_trait;
use warp::{Filter, Rejection, Reply};

use crate::{Pool, main};
use crate::usecases::device_logs::UsesDeviceLogUseCase;

//pub struct DeviceLogHandler<T> where T: UsesDeviceLogUseCase {
//  usecase: T,
//}
//
//impl<T: UsesDeviceLogUseCase + Sync> DeviceLogHandler<T> {
//  async fn device_log_handler(&self, pool: web::Data<Pool>) -> HttpResponse where T: UsesDeviceLogUseCase {
//     let _ = self.usecase.provide_device_log_usecase();
//     let name = "";
//     HttpResponse::Ok()
//     .content_type("text/plain")
//     .body(format!("Hello {}!", name))
//  }
//}

//pub struct DeviceLogHandler {}
//
//impl UsesDeviceLogUseCase for DeviceLogHandler {
//  type DeviceLogUseCase = Self;
//
//  fn provide_device_log_usecase(self: Box<Self>) -> Self::DeviceLogUseCase {
//    self
//  }
//}

//pub fn list<A: TodoController + Send + Sync + 'static>(
//  web: web::Data<A>,
//) -> impl Future01<Item = web::Json<Vec<Todo>>, Error = Error> {
//  let f_resp = async move {
//      let controller = web.get_ref();
//      let listed = controller.list().await;
//      Ok(web::Json(listed))
//  };
//  f_resp.boxed().compat()
//}

#[async_trait]
pub trait DeviceLogHandler: UsesDeviceLogUseCase + Sync {
    async fn get_device_logs(&self) {
      self.provide_device_log_usecase();
    }
}

impl<T: UsesDeviceLogUseCase + Sync> DeviceLogHandler for T {}

pub trait UsesDeviceLogHandler {
  type DeviceLogHandler: DeviceLogHandler;

  fn provide_device_log_handler(&self) -> Self::DeviceLogHandler;
}
