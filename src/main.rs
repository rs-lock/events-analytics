use actix_web::{
    App, HttpServer,
    middleware::Logger,
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
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let api_bind = env("INGESTION_BIND");
    let kafka_bind = env("KAFKA_BROKERS");

    let server = HttpServer::new(move || {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &kafka_bind)
            .set("message.timeout.ms", "1000")
            .set("compression.type", "lz4")
            .set("linger.ms", "10")
            .set("batch.num.messages", "10000")
            .set("acks", "1")
            .create()
            .expect("Producer creation error");

        let data = web::Data::new(producer);
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1").route("/events/{event_type}", web::post().to(handle_event)),
            )
            .route("/health", web::get().to(health))
            .app_data(data)
    })
    .bind(&api_bind)?
    .shutdown_timeout(30);

    tracing::info!(bind = %api_bind, "Ingestion API started");
    server.run().await
}
