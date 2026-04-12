use std::time::Duration;

use crate::{errors::EventError, models::Event, validator::validate_event_type};
use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use rdkafka::producer::{FutureProducer, FutureRecord};

pub async fn handle_event(
    path: Path<String>,
    event: Json<Event>,
    producer: Data<FutureProducer>,
) -> Result<HttpResponse, EventError> {
    println!("Получено событие: {:?}", event);
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
        .send(record, Duration::from_millis(450))
        .await
        .map_err(|_| EventError::KafkaError)?;
    let ok = HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }));
    Result::Ok(ok)
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}
