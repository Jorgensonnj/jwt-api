use super::back_end::module_service_config::back_end_module_service_config;

use actix_web::{web, web::ServiceConfig};

pub fn full_stack_service_config(config: &mut ServiceConfig) {
    config
        .service(
            web::scope("")
                .configure(back_end_module_service_config)
        );
}


