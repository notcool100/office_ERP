use be::{api::user::vmail::MailcowConfig, build_routes, init_pool, middleware};

use axum::{
    extract::DefaultBodyLimit,
    http::{HeaderValue, header::CACHE_CONTROL},
};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db_pool = match init_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to init DB pool: {}", e);
            eprintln!("Failed to init DB pool: {}", e);
            // We still need a pool for the app to function, but we can return the error
            // or exit gracefully if it's strictly required.
            // For now, let's keep the panic but with a better message,
            // OR make it optional if the user wants to just see the server start.
            panic!(
                "Critical: Failed to connect to primary database. Please check your DATABASE_URL in .env (if you are local, try using localhost). Error: {}",
                e
            );
        }
    };

    if let Err(e) = sqlx::migrate!("./migrations").run(&db_pool).await {
        tracing::error!("Failed to run database migrations: {}", e);
        panic!("Critical: Failed to run database migrations: {}", e);
    }

    let mailcow_config = MailcowConfig::from_env();
    if mailcow_config.is_none() {
        tracing::warn!(
            "MAILCOW_API_URL / MAILCOW_API_KEY not set — mailbox provisioning will be skipped"
        );
    }

    // Start background scheduler for due-date reminders
    be::scheduler::start(db_pool.clone());

    let hub = be::ws::hub::Hub::new();

    use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT]);

    let app = build_routes(hub.clone())
        .layer(middleware::add_extensions(db_pool, mailcow_config))
        .layer(cors)
        .layer(SetResponseHeaderLayer::overriding(
            CACHE_CONTROL,
            HeaderValue::from_static(
                "no-store, no-cache, must-revalidate, proxy-revalidate, max-age=0",
            ),
        ))
        .layer(DefaultBodyLimit::disable())
        .layer(TraceLayer::new_for_http());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3117".to_string())
        .parse()
        .expect("PORT must be a number");

    // Bind to 0.0.0.0 to allow external access
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::debug!("Listening on {}", addr);
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
