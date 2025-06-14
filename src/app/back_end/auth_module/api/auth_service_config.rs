use super::routes::login;
use actix_web::{web::get, web::ServiceConfig};

pub fn auth_api_service(config: &mut ServiceConfig) {
    config
        .route("/login", get().to(login));
}

