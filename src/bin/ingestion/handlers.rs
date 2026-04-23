use std::time::Duration;

use crate::validator::validate_event_type;
use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use event_analytics::{errors::EventError, models::Event};
use rdkafka::producer::{FutureProducer, FutureRecord};

#[post("/events/{event_type}")]
pub async fn handle_event(
    path: Path<String>,
    event: Json<Event>,
    producer: Data<FutureProducer>,
) -> Result<HttpResponse, EventError> {
    let path = path.into_inner();
    let event_type = validate_event_type(&path)?;

    let event = event.into_inner();
    let string_event = serde_json::to_string(&event).map_err(|_| EventError::SerError)?;
    let string_uid = event.user_id.to_string();

    let topic = event_type.as_topic();
    let record = FutureRecord::to(&topic)
        .payload(&string_event)
        .key(&string_uid);

    producer
        .send(record, Duration::from_millis(10))
        .await
        .map_err(|_| EventError::KafkaError)?;
    let ok = HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }));
    Result::Ok(ok)
}

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}
