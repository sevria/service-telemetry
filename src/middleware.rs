use std::sync::Arc;

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

    let response = next.run(request).await;

    let status = response.status();

    telemetry
        .metrics
        .record_request(method, path.to_string().as_str(), status);

    response
}
