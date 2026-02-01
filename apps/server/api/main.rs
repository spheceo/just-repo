use axum::{
    Json, Router,
    http::header,
    response::IntoResponse,
    routing::get,
};
use dotenvy::dotenv;
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use vercel_runtime::axum::VercelLayer;

async fn favicon() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/x-icon")],
        include_bytes!("../public/favicon.ico").as_slice(),
    )
}

async fn hello() -> impl IntoResponse {
    Json(json!({ "message": "Welcome to the just-repo project!" }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load .env file
    dotenv().ok();

    // Add CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/", get(hello))
        .route("/favicon.ico", get(favicon))
        .layer(cors);

    // Use the Vercel port in prod but use 8080 locally.
    if std::env::var("VERCEL").is_ok() {
        let app = ServiceBuilder::new()
            .layer(VercelLayer::new())
            .service(router);
        vercel_runtime::run(app).await?;
    } else {
        let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
        let addr = format!("0.0.0.0:{}", port);
        println!("Server running at http://localhost:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, router).await?;
    }
    Ok(())
}
