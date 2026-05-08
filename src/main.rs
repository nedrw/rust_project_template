//! {{project_description}}

{% if use_logging or use_tracing -%}
// === Async boot sequence (logging/tracing requires tokio) ===

{% if use_settings or use_cli %}mod settings;{% endif %}
mod telemetry;
mod metrics;

use foundations::service_info;
use foundations::BootstrapResult;
{% if use_cli %}
use foundations::cli::Cli;
{% endif %}
use foundations::telemetry::log;

// When the `jemalloc` feature is enabled in Cargo.toml, foundations
// automatically sets jemalloc as the global allocator. No manual
// #[global_allocator] declaration is needed.

#[tokio::main]
async fn main() -> BootstrapResult<()> {
    let service_info = service_info!();

    {% if use_cli %}
    let cli = Cli::<settings::AppSettings>::new(&service_info, vec![])?;
    let settings = &cli.settings;
    {% elsif use_settings %}
    let settings = settings::AppSettings::default();
    {% endif %}

    let tele_driver = telemetry::init(
        &service_info{% if use_cli %}, settings{% elsif use_settings %}, &settings{% endif %}
    )?;

    log::info!("Service started successfully");

    // TODO: Add your business logic here

    tele_driver.await?;
    Ok(())
}

{% elsif use_cli or use_settings or use_jemalloc -%}
// === Sync boot sequence (settings/CLI/jemalloc, no async runtime needed) ===

{% if use_settings or use_cli %}mod settings;{% endif %}

use foundations::service_info;
use foundations::BootstrapResult;
{% if use_cli %}
use foundations::cli::Cli;
{% endif %}

// When the `jemalloc` feature is enabled in Cargo.toml, foundations
// automatically sets jemalloc as the global allocator. No manual
// #[global_allocator] declaration is needed.

fn main() -> BootstrapResult<()> {
    let service_info = service_info!();

    {% if use_cli %}
    let cli = Cli::<settings::AppSettings>::new(&service_info, vec![])?;
    let _settings = &cli.settings;
    {% elsif use_settings %}
    let _settings = settings::AppSettings::default();
    {% endif %}

    // TODO: Add your business logic here

    Ok(())
}

{% else -%}
// === Minimal project (no Foundations features selected) ===

fn main() {
    println!("Hello, world!");
}
{% endif -%}
