use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::{
    logs::SdkLoggerProvider,
    metrics::{PeriodicReader, SdkMeterProvider},
    propagation::TraceContextPropagator,
    trace::SdkTracerProvider,
    Resource,
};
use opentelemetry_semantic_conventions::attribute::SERVICE_NAME;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tonic::metadata::MetadataMap;
use std::env;

pub struct TelemetryProviders {
    pub tracer_provider: SdkTracerProvider,
    pub logger_provider: SdkLoggerProvider,
    pub meter_provider: SdkMeterProvider,
}

impl TelemetryProviders {
    pub fn shutdown(self) {
        if let Err(e) = self.tracer_provider.shutdown() { eprintln!("TracerProvider shutdown error: {e}"); }
        if let Err(e) = self.logger_provider.shutdown() { eprintln!("LoggerProvider shutdown error: {e}"); }
        if let Err(e) = self.meter_provider.shutdown()  { eprintln!("MeterProvider shutdown error: {e}"); }
    }
}

fn resource() -> Resource {
    let service_name = env::var("OTEL_SERVICE_NAME")
        .unwrap_or_else(|_| "rust-just-learn".to_string());
    Resource::builder()
        .with_attribute(opentelemetry::KeyValue::new(SERVICE_NAME, service_name))
        .build()
}

fn endpoint() -> String {
    env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:4317".to_string())
}

/// Build gRPC metadata with Authorization header from HYPERDX_API_KEY (if set)
fn otlp_metadata() -> MetadataMap {
    let mut map = MetadataMap::new();
    if let Ok(api_key) = env::var("HYPERDX_API_KEY") {
        if !api_key.is_empty() {
            if let Ok(v) = api_key.parse() {
                map.insert("authorization", v);
            }
        }
    }
    map
}

pub fn init_telemetry() -> Result<TelemetryProviders, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let endpoint = endpoint();
    let resource = resource();
    let metadata = otlp_metadata();

    // ---- Tracer ----
    global::set_text_map_propagator(TraceContextPropagator::new());
    let span_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(&endpoint)
        .with_metadata(metadata.clone())
        .build()?;
    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .with_resource(resource.clone())
        .build();
    let tracer = tracer_provider.tracer("rust-just-learn");
    global::set_tracer_provider(tracer_provider.clone());

    // ---- Logger ----
    let log_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .with_endpoint(&endpoint)
        .with_metadata(metadata.clone())
        .build()?;
    let logger_provider = SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .with_resource(resource.clone())
        .build();

    // ---- Meter ----
    let metric_exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_endpoint(&endpoint)
        .with_metadata(metadata)
        .build()?;
    let meter_provider = SdkMeterProvider::builder()
        .with_reader(
            PeriodicReader::builder(metric_exporter)
                .with_interval(std::time::Duration::from_secs(10))
                .build(),
        )
        .with_resource(resource)
        .build();
    global::set_meter_provider(meter_provider.clone());

    // ---- Subscriber ----
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(&logger_provider))
        .with(tracing_subscriber::fmt::layer().with_ansi(true))
        .try_init()?;

    Ok(TelemetryProviders { tracer_provider, logger_provider, meter_provider })
}
