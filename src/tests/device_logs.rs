#[cfg(test)]
mod tests {
    use crate::handlers::{get_devicelog_handler, post_devicelog_handler};
    use crate::models::{DeviceLogPayload, DeviceLogs};
    use actix_web::test;
    use actix_web::{web, App};
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    #[actix_rt::test]
    async fn test_get_device_logs() {
        let mut app = test::init_service(
            App::new().service(
                web::scope("/api/v1").service(
                    web::resource("/device_logs")
                        .route(web::get().to(get_devicelog_handler))
                        .route(web::post().to(post_devicelog_handler)),
                ),
            ),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/v1/device_logs")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_post_device_logs() {
        let mut app = test::init_service(
            App::new().service(
                web::scope("/api/v1").service(
                    web::resource("/device_logs")
                        .route(web::get().to(get_devicelog_handler))
                        .route(web::post().to(post_devicelog_handler)),
                ),
            ),
        )
        .await;

        let d = NaiveDate::from_ymd(2021, 3, 3);
        let t = NaiveTime::from_hms_milli(12, 35, 56, 789);
        let dt = NaiveDateTime::new(d, t);

        let payload = DeviceLogPayload {
            device_name: "test device".to_string(),
            location_name: "test location".to_string(),
            scanned_name: Some("test android".to_string()),
            scanned_id: "".to_string(),
            scanned_rssi: 100,
            scanned_time: dt,
        };

        let logs = DeviceLogs {
            device_logs: vec![payload],
        };

        let req = test::TestRequest::post()
            .set_json(&logs)
            .uri("/api/v1/device_logs")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }
}
