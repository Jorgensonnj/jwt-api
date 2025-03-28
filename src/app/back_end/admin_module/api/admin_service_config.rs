use super::routes::status;
use actix_web::{web::get, web::ServiceConfig};

pub fn admin_api_service(config: &mut ServiceConfig) {
    config
        .route("/status", get().to(status));
}

