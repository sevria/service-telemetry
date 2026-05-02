mod init;
mod metrics;
mod middleware;

pub use init::init;
pub use metrics::Metrics;
pub use middleware::telemetry_middleware;

pub struct Telemetry {
    pub metrics: Metrics,
}
