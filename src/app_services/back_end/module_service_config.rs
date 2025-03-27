use super::{
    admin_module::version_service_config::admin_version_service_config,
    auth_module::version_service_config::auth_version_service_config
};
use actix_web::{web::{self, scope, ServiceConfig}, HttpResponse};
use actix_web_grants::AuthorityGuard;

pub fn back_end_module_service_config(config: &mut ServiceConfig) {

    let admin_service = scope("/admin")
        .service(
            scope("")
                .guard(AuthorityGuard::new("yikes".to_string()))
                .configure(admin_version_service_config)
        )
        .default_service(
            web::head().to(HttpResponse::Forbidden)
        );

    let auth_service = scope("/auth")
        .configure(auth_version_service_config);

    config
        .service(admin_service)
        .service(auth_service);
}
