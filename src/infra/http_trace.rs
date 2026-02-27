//! HTTP tracing & metrics middleware for Axum.
//! Lives in `infra` because it's infrastructure-level instrumentation,
//! not business logic.

use opentelemetry::{global, trace::Status as OtelStatus, KeyValue};
use tracing_opentelemetry::OpenTelemetrySpanExt;

// ---- Span status + auto metrics -----------------------------------------

/// Sets the OTel span status from HTTP response status, and records metrics:
/// - `http_requests_total` (counter)
/// - `http_request_duration_ms` (histogram)
#[derive(Clone)]
pub struct OtelOnResponse;

impl<B> tower_http::trace::OnResponse<B> for OtelOnResponse {
    fn on_response(
        self,
        response: &axum::http::Response<B>,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        let http_status = response.status();
        let status_str = http_status.as_u16().to_string();
        let latency_ms = latency.as_millis() as f64;

        // --- OTel span status ---
        if http_status.is_server_error() {
            span.set_status(OtelStatus::error(format!("HTTP {}", http_status.as_u16())));
        } else {
            span.set_status(OtelStatus::Ok);
        }

        // --- metrics  (auto for every request) ---
        let meter = global::meter("rust-just-learn");
        let attrs = &[KeyValue::new("http.status_code", status_str.clone())];

        meter
            .u64_counter("http_requests_total")
            .with_description("Total HTTP requests")
            .build()
            .add(1, attrs);

        meter
            .f64_histogram("http_request_duration_ms")
            .with_description("HTTP request latency in milliseconds")
            .with_unit("ms")
            .build()
            .record(latency_ms, attrs);

        // Separate histogram — error requests only (5xx)
        if http_status.is_server_error() {
            meter
                .f64_histogram("http_error_duration_ms")
                .with_description("Latency of HTTP 5xx error requests")
                .with_unit("ms")
                .build()
                .record(latency_ms, attrs);
        }

        tracing::info!(
            http.status_code = http_status.as_u16(),
            latency_ms = latency_ms,
            "HTTP {} in {}ms",
            http_status.as_u16(),
            latency_ms as u64,
        );
    }
}

// ---- Metrics helpers -------------------------------------------------------

/// Manually increment `http_requests_total` with route + method labels,
/// call this inside handlers if you need finer-grained route tagging.
pub fn record_http_request(route: &str, method: &str, status: u16) {
    let meter = global::meter("rust-just-learn");
    meter
        .u64_counter("http_requests_total")
        .with_description("Total HTTP requests")
        .build()
        .add(
            1,
            &[
                KeyValue::new("route", route.to_string()),
                KeyValue::new("method", method.to_string()),
                KeyValue::new("http.status_code", status.to_string()),
            ],
        );
}
