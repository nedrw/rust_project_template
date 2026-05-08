//! Application settings.
//!
//! All settings have sensible defaults, so the service works out of the box.
//! Run with `--help` to see available options, or `-g <path>` to generate
//! a default YAML configuration file with full documentation.

use foundations::settings::settings;

{% if use_logging or use_tracing %}
use foundations::telemetry::settings::TelemetrySettings;
{% endif %}

/// Top-level application settings.
///
/// All fields have defaults, so a minimal config file or no config at all
/// will produce a working service.
#[settings]
pub struct AppSettings {
    {% if use_logging or use_tracing %}
    /// Telemetry configuration (logging, distributed tracing, metrics, telemetry server).
    #[serde(default)]
    pub telemetry: TelemetrySettings,
    {% endif %}

    // Add your custom settings fields below.
    //
    // Example:
    // /// The TCP address the service listens on.
    // #[serde(default = "AppSettings::default_listen_addr")]
    // pub listen_addr: foundations::settings::net::SocketAddr,
}

impl AppSettings {
    // Example default function — uncomment when you add fields above:
    // fn default_listen_addr() -> foundations::settings::net::SocketAddr {
    //     foundations::settings::net::SocketAddr::from_str("127.0.0.1:8080").unwrap()
    // }
}
