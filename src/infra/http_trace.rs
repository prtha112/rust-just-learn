//! HTTP tracing & metrics middleware for Axum.
//! Metric names follow OTel Semantic Conventions:
//! https://opentelemetry.io/docs/specs/semconv/http/http-metrics/

use opentelemetry::{global, trace::Status as OtelStatus, KeyValue};
use tracing_opentelemetry::OpenTelemetrySpanExt;

// ---- Span status + auto metrics -----------------------------------------

/// Sets OTel span status and records standard HTTP server metrics.
///
/// Metrics emitted (per OTel Semantic Conventions):
/// - `http.server.request.duration` (histogram, seconds)
/// - `http.server.error.duration`   (histogram, seconds — 5xx only)
///
/// Attributes:
/// - `http.response.status_code`
/// - `http.request.method`  (if accessible via span)
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
        let status_code = http_status.as_u16();
        // OTel standard: duration in SECONDS (not milliseconds)
        let duration_secs = latency.as_secs_f64();

        // --- OTel span status ---
        if http_status.is_server_error() {
            span.set_status(OtelStatus::error(format!("HTTP {}", status_code)));
        } else {
            span.set_status(OtelStatus::Ok);
        }

        // --- OTel standard attributes ---
        let attrs = &[KeyValue::new("http.response.status_code", status_code as i64)];
        let meter = global::meter("rust-just-learn");

        // http.server.request.duration — REQUIRED by OTel spec
        meter
            .f64_histogram("http.server.request.duration")
            .with_description("Duration of HTTP server requests")
            .with_unit("s")
            .build()
            .record(duration_secs, attrs);

        // http.server.error.duration — 5xx only (custom extension)
        if http_status.is_server_error() {
            meter
                .f64_histogram("http.server.error.duration")
                .with_description("Duration of HTTP 5xx server error requests")
                .with_unit("s")
                .build()
                .record(duration_secs, attrs);
            // --- log event inside span ---
            /*
            tracing::error!(
                "http.response.status_code" = status_code,
                latency_ms = latency.as_millis(),
                "HTTP {} in {}ms",
                status_code,
                latency.as_millis(),
            );
            */
        } 
        /*
        else {
            // --- log event inside span ---
            tracing::info!(
                "http.response.status_code" = status_code,
                latency_ms = latency.as_millis(),
                "HTTP {} in {}ms",
                status_code,
                latency.as_millis(),
            );
        }
        */
    }
}

// ---- Metrics helpers -------------------------------------------------------

/// Manually record a request with full OTel-standard attributes.
/// Use when you need finer-grained `http.route` labeling per handler.
pub fn record_http_request(method: &str, route: &str, status: u16, duration_secs: f64) {
    let meter = global::meter("rust-just-learn");
    let attrs = &[
        KeyValue::new("http.request.method", method.to_string()),
        KeyValue::new("http.route", route.to_string()),
        KeyValue::new("http.response.status_code", status as i64),
    ];
    meter
        .f64_histogram("http.server.request.duration")
        .with_description("Duration of HTTP server requests")
        .with_unit("s")
        .build()
        .record(duration_secs, attrs);
}
