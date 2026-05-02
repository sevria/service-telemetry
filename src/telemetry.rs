use anyhow::Result;
use gethostname::gethostname;
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::MetricExporter;
use opentelemetry_sdk::{Resource, metrics::SdkMeterProvider};
use opentelemetry_semantic_conventions::resource::SERVICE_INSTANCE_ID;

pub fn init(service_name: &str) -> Result<()> {
    let hostname = gethostname().to_string_lossy().to_string();
    let resource = Resource::builder()
        .with_service_name(service_name.to_string())
        .with_attribute(KeyValue::new(SERVICE_INSTANCE_ID, hostname))
        .build();

    init_meter_provider(resource)?;

    Ok(())
}

fn init_meter_provider(resource: Resource) -> Result<SdkMeterProvider> {
    let exporter = MetricExporter::builder().with_tonic().build()?;
    let provider = SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(resource)
        .build();

    global::set_meter_provider(provider.clone());

    Ok(provider)
}
