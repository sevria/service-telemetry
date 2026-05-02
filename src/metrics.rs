use http::{Method, StatusCode};
use opentelemetry::{KeyValue, global, metrics::Counter};
use opentelemetry_semantic_conventions::attribute::{
    HTTP_REQUEST_METHOD, HTTP_RESPONSE_STATUS_CODE, HTTP_ROUTE,
};

#[derive(Clone)]
pub struct Metrics {
    pub request_count: Counter<u64>,
}

impl Metrics {
    pub fn new() -> Self {
        let meter = global::meter("telemetry");

        let request_count = meter
            .u64_counter("http.server.request.count")
            .with_description("Total number of HTTP requests received")
            .with_unit("requests")
            .build();

        Self { request_count }
    }

    pub fn record_request(&self, method: Method, path: &str, status: StatusCode) {
        let attributes = [
            KeyValue::new(HTTP_REQUEST_METHOD, method.to_string()),
            KeyValue::new(HTTP_ROUTE, path.to_string()),
            KeyValue::new(HTTP_RESPONSE_STATUS_CODE, status.as_u16().to_string()),
        ];

        self.request_count.add(1, &attributes);
    }
}
