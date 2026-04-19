use actix_web::{
    App, HttpServer,
    web::{self},
};
use event_analytics::env;
use rdkafka::{ClientConfig, producer::FutureProducer};

use crate::handlers::{handle_event, health};

mod errors;
mod handlers;
mod validator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let api_bind = env("INGESTION_BIND");
    let kafka_bind = env("KAFKA_BROKERS");

    HttpServer::new(move || {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &kafka_bind)
            .set("message.timeout.ms", "1000")
            .create()
            .expect("Producer creation error");

        let data = web::Data::new(producer);
        App::new()
            .service(
                web::scope("/api/v1").route("/events/{event_type}", web::post().to(handle_event)),
            )
            .route("/health", web::get().to(health))
            .app_data(data)
    })
    .bind(api_bind)?
    .run()
    .await
}
