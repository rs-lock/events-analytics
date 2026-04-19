use std::io::Result;

use actix_web::{
    App, HttpServer,
    web::{self},
};

use event_analytics::env;

use clickhouse::Client;
mod errors;
mod handlers;
mod models;
mod queries;
mod response;
mod sql;
mod validation;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let ch_url = env("CLICKHOUSE_URL");
    let ch_user = env("CLICKHOUSE_USER");
    let ch_psw = env("CLICKHOUSE_PASSWORD");
    let ch_db = env("CLICKHOUSE_DATABASE");

    let api_bind = env("ANALYTICS_BIND");

    let client = Client::default()
        .with_url(ch_url)
        .with_user(ch_user)
        .with_password(ch_psw)
        .with_database(ch_db);

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api/v1")
                    .route(
                        "/analytics/top-products",
                        web::get().to(handlers::handle_top_products),
                    )
                    .route(
                        "/analytics/user-activity/{user_id}",
                        web::get().to(handlers::handle_user_activity),
                    )
                    .route(
                        "/analytics/conversion-rate",
                        web::get().to(handlers::handle_conversion_rate),
                    )
                    .route(
                        "/analytics/realtime-stats",
                        web::get().to(handlers::handle_realtime_stats),
                    ),
            )
            .app_data(web::Data::new(client.clone()))
    })
    .bind(api_bind)?
    .run()
    .await
}
