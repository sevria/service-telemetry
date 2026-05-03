use std::sync::Arc;
use std::time::Instant;

use axum::{
    extract::{MatchedPath, Request, State},
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
    let mut path = "".to_string();
    let started_at = Instant::now();

    if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
        path = matched_path.as_str().to_string();
    }

    let response = next.run(request).await;

    let status = response.status();
    let duration = started_at.elapsed().as_secs_f64();

    telemetry
        .metrics
        .after_handle(&method, &path, &status, duration);

    response
}
