use std::sync::Arc;
use std::time::Instant;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::Telemetry;

pub async fn telemetry_middleware(
    State(telemetry): State<Arc<Telemetry>>,
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let path = request.uri().clone();
    let path = path.to_string();
    let started_at = Instant::now();

    telemetry.metrics.before_handle(&method, &path);

    let response = next.run(request).await;

    let status = response.status();
    let duration = started_at.elapsed().as_secs_f64();

    telemetry
        .metrics
        .after_handle(&method, &path, &status, duration);

    response
}
