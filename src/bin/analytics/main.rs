use std::io::Result;

use actix_web::{
    App, HttpServer,
    web::{self},
};
use clickhouse::Client;
mod errors;
mod handlers;
mod models;
mod sql;
mod validation;

#[actix_web::main]
async fn main() -> Result<()> {
    let client = Client::default()
        .with_url("http://localhost:8123")
        .with_user("default")
        .with_password("")
        .with_database("events");

    HttpServer::new(move || {
        App::new()
            .service(web::scope("/api/v1").route(
                "/analytics/top-products",
                web::get().to(handlers::handle_top_products),
            ))
            .app_data(web::Data::new(client.clone()))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
