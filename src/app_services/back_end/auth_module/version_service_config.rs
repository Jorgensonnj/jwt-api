use super::api::auth_service_config::auth_api_service;
use actix_web::web::{scope, ServiceConfig};

pub fn auth_version_service_config(config: &mut ServiceConfig) {
    config
        .service(
            scope("/api/v1")
                .configure(auth_api_service)
        );
}
