use super::{
    admin_module::version_service_config::admin_version_service_config,
    auth_module::version_service_config::auth_version_service_config
};

use actix_web::web::{scope, ServiceConfig};

pub fn back_end_module_service_config(config: &mut ServiceConfig) {
    config
        .service(
        scope("/admin")
            .configure(admin_version_service_config)
        )
        .service(
            scope("/auth")
                .configure(auth_version_service_config)
        );
}
