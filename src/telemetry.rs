//! Telemetry initialization.
//!
//! This module provides helpers to initialize logging, distributed tracing,
//! metrics, and the telemetry server — all driven by `TelemetryDriver`.

use foundations::telemetry::{TelemetryConfig, TelemetryDriver};
{% if use_settings == false and use_cli == false %}
use foundations::telemetry::settings::TelemetrySettings;
{% endif %}
use foundations::{BootstrapResult, ServiceInfo};

{% if use_settings or use_cli %}
use crate::settings::AppSettings;
{% endif %}

/// Initialize telemetry and return the driver that must be `.await`ed
/// to keep telemetry running for the lifetime of the service.
///
/// If settings are available, they will be used to configure telemetry;
/// otherwise sensible defaults are applied.
pub fn init(
    service_info: &ServiceInfo,
    {% if use_settings or use_cli %}
    settings: &AppSettings,
    {% endif %}
) -> BootstrapResult<TelemetryDriver> {
    {% if use_settings == false and use_cli == false %}
    // No app settings available — use default telemetry configuration
    let default_telemetry = TelemetrySettings::default();
    {% endif %}

    let config = TelemetryConfig {
        service_info,
        settings: {% if use_settings or use_cli %}&settings.telemetry{% else %}&default_telemetry{% endif %},
        custom_server_routes: vec![],
    };

    let driver = foundations::telemetry::init(config)?;

    if let Some(addr) = driver.server_addr() {
        use foundations::addr::ListenAddr;
        match addr {
            ListenAddr::Tcp(addr) => {
                foundations::telemetry::log::info!("Telemetry server listening on http://{addr}");
            }
            ListenAddr::Unix(path) => {
                foundations::telemetry::log::info!("Telemetry server listening on {path:?}");
            }
        }
    }

    Ok(driver)
}
