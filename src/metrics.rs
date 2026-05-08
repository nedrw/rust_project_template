//! Custom service metrics.
//!
//! Define your application-specific metrics here using the `#[metrics]` macro.
//! Metrics are automatically registered with the telemetry server and exposed
//! via the `/metrics` endpoint.
//!
//! Supported metric types:
//! - `Counter` — monotonically increasing value (e.g., request count)
//! - `Gauge` — value that can go up and down (e.g., active connections)
//! - `Histogram` — distribution of values (e.g., request latency)
//!
//! Each metric function accepts labels as parameters, enabling fine-grained
//! filtering in Prometheus queries.

use foundations::telemetry::metrics::metrics;

/// Metrics for the application.
///
/// Uncomment the example function signatures below and replace with your own
/// application-specific metrics. The function name becomes the metric name,
/// and parameters become metric labels.
#[metrics]
pub mod app {
    // Example metric definitions:
    //
    // /// Total number of requests processed.
    // pub fn requests_total(endpoint: &str) -> Counter;
    //
    // /// Number of currently active connections.
    // pub fn active_connections() -> Gauge;
}
