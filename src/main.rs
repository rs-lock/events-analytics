use actix_web::{
    App, HttpServer,
    web::{self},
};
use rdkafka::{ClientConfig, producer::FutureProducer};

use crate::handlers::{handle_event, health};

mod errors;
mod handlers;

mod models;
mod validator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
