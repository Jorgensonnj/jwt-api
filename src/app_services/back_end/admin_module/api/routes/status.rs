use actix_web::{HttpRequest, HttpResponse, Responder};
use tracing::instrument;

#[instrument]
pub async fn status(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "alive"}))
}
