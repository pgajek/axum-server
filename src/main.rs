use axum::{
    http::Request,
    middleware::{self, Next},
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let config = RustlsConfig::from_pem_file("cert.pem", "key.pem")
        .await
        .expect("Failed to load TLS configuration");

    let app = Router::new()
        .route("/", get(root))
        .layer(middleware::from_fn(log_requests));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8123));
    println!("Listening on https://{}", addr);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .expect("Server failed");

    Ok(())
}

async fn log_requests(
    req: Request<axum::body::Body>,
    next: Next,
) -> impl axum::response::IntoResponse {
    info!("Received request: {} {}", req.method(), req.uri());
    next.run(req).await
}

async fn root() -> &'static str {
    "Welcome to the secure Axum server!"
}
