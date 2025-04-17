use super::{
    super::shared::models::auth_models::{
        Module, AccessLevel, ModulePermission
    },
    admin_module::version_service_config::admin_version_service_config,
    auth_module::version_service_config::auth_version_service_config
};
use actix_web::{guard::{self, AnyGuard}, web::{self, scope, ServiceConfig}, HttpResponse};
use actix_web_grants::AuthorityGuard;

pub fn back_end_module_service_config(config: &mut ServiceConfig) {

    let admin_service = scope("/admin")
        .service(
            scope("")
                .guard(admin_guard())
                .configure(admin_version_service_config)
        )
        .default_service(
            web::head().to(HttpResponse::Forbidden)
        );

    let auth_service = scope("/auth")
        .configure(auth_version_service_config);

    // add services
    config
        .service(admin_service)
        .service(auth_service);
}

fn admin_guard() -> AnyGuard {
    let admin_read_only_guard = AuthorityGuard::new(
            ModulePermission {
                module: Module::Admin,
                access_level: AccessLevel::ReadOnly
            }
        );
    let admin_read_write_guard = AuthorityGuard::new(
            ModulePermission {
                module: Module::Admin,
                access_level: AccessLevel::ReadAndWrite
            }
        );

    guard::Any(admin_read_only_guard)
        .or(admin_read_write_guard)
}

