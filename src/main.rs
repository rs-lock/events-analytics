use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{self},
};

use crate::handlers::{handle_event, health};

mod handlers;
mod models;
mod validator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1").route("/events/{event_type}", web::post().to(handle_event)),
            )
            .route("/health", web::get().to(health))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
