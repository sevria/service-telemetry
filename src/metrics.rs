use http::{Method, StatusCode};
use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram, UpDownCounter},
};
use opentelemetry_semantic_conventions::{
    attribute::{HTTP_REQUEST_METHOD, HTTP_RESPONSE_STATUS_CODE, HTTP_ROUTE},
    metric::{HTTP_SERVER_ACTIVE_REQUESTS, HTTP_SERVER_REQUEST_DURATION},
};

#[derive(Clone)]
pub struct Metrics {
    pub request_count: Counter<u64>,
    pub active_requests: UpDownCounter<i64>,
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
            active_requests: meter
                .i64_up_down_counter(HTTP_SERVER_ACTIVE_REQUESTS)
                .with_description("Number of in-flight requests currently being handled")
                .with_unit("requests")
                .build(),
            request_duration: meter
                .f64_histogram(HTTP_SERVER_REQUEST_DURATION)
                .with_description("HTTP request duration in seconds")
                .with_unit("s")
                .build(),
        }
    }

    pub fn before_handle(&self, method: &Method, path: &str) {
        let attributes = [
            KeyValue::new(HTTP_REQUEST_METHOD, method.to_string()),
            KeyValue::new(HTTP_ROUTE, path.to_string()),
        ];

        self.active_requests.add(1, &attributes);
    }

    pub fn after_handle(&self, method: &Method, path: &str, status: &StatusCode, duration: f64) {
        let attributes = [
            KeyValue::new(HTTP_REQUEST_METHOD, method.to_string()),
            KeyValue::new(HTTP_RESPONSE_STATUS_CODE, status.as_u16().to_string()),
            KeyValue::new(HTTP_ROUTE, path.to_string()),
        ];

        self.active_requests.add(-1, &attributes);
        self.request_count.add(1, &attributes);
        self.request_duration.record(duration, &attributes);
    }
}
