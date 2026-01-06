use be::{
    build_routes,
    init_pool,
    middleware,
};

use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::{cors::{CorsLayer, Any}, trace::TraceLayer};
use tracing_subscriber::EnvFilter;



#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    tracing::info!("Starting Axum server...");

    let db_pool = init_pool().await.expect("Failed to init DB pool");

    use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT]);

    let app = build_routes()
        .layer(middleware::add_extensions(db_pool))
        .layer(cors)
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