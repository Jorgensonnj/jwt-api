use super::api::admin_service_config::admin_api_service;
use actix_web::web::{scope, ServiceConfig};

pub fn admin_version_service_config(config: &mut ServiceConfig) {
    config
        .service(
            scope("/api/v1")
                .configure(admin_api_service)
        );
}
