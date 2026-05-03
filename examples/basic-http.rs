use anyhow::Result;
use axum::{Router, extract::Path, middleware, response::IntoResponse, routing::get};
use http::StatusCode;
use sevria_service_telemetry::telemetry_middleware;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let telemetry = sevria_service_telemetry::init("basic-http")?;
    let router = Router::new()
        .route("/", get(hello))
        .route("/say-hello/{name}", get(say_hello))
        .route("/oops", get(oops))
        .layer(middleware::from_fn_with_state(
            telemetry,
            telemetry_middleware,
        ));
    let address = "127.0.0.1:3000";
    let listener = TcpListener::bind(address).await?;

    println!("Running HTTP server on {}", address);

    axum::serve(listener, router).await?;

    Ok(())
}

async fn hello() -> &'static str {
    "Hello, world!"
}

async fn say_hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

async fn oops() -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Oops! Something went wrong.",
    )
}
