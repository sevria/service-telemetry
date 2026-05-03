use http::{Method, StatusCode};
use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram},
};
use opentelemetry_semantic_conventions::{
    attribute::{HTTP_REQUEST_METHOD, HTTP_RESPONSE_STATUS_CODE, HTTP_ROUTE},
    metric::HTTP_SERVER_REQUEST_DURATION,
};

#[derive(Clone)]
pub struct Metrics {
    pub request_count: Counter<u64>,
    pub request_duration: Histogram<f64>,
}

impl Metrics {
    pub fn new() -> Self {
        let meter = global::meter("telemetry");

        Self {
            request_count: meter
                .u64_counter("http.server.request.count")
                .with_description("Total number of HTTP requests received")
                .with_unit("requests")
                .build(),
            request_duration: meter
                .f64_histogram(HTTP_SERVER_REQUEST_DURATION)
                .with_description("HTTP request duration in seconds")
                .with_unit("s")
                .with_boundaries(vec![
                    0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0,
                ])
                .build(),
        }
    }

    pub fn after_handle(&self, method: &Method, path: &str, status: &StatusCode, duration: f64) {
        let attributes = [
            KeyValue::new(HTTP_REQUEST_METHOD, method.to_string()),
            KeyValue::new(HTTP_RESPONSE_STATUS_CODE, status.as_u16().to_string()),
            KeyValue::new(HTTP_ROUTE, path.to_string()),
        ];

        self.request_count.add(1, &attributes);
        self.request_duration.record(duration, &attributes);
    }
}
