use crate::{models::Event, validator::validate_by};
use actix_web::{HttpResponse, web::Json, web::Path};

pub async fn handle_event(path: Path<String>, event: Json<Event>) -> HttpResponse {
    println!("Получено событие: {:?}", event);
    validate_by(path.into_inner());
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}
