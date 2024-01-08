mod logger;

use opentelemetry::{global, KeyValue};

use opentelemetry_sdk::Resource;

use opentelemetry_semantic_conventions::resource::{
    DEPLOYMENT_ENVIRONMENT, SERVICE_NAME, SERVICE_VERSION,
};
use opentelemetry_semantic_conventions::SCHEMA_URL;

use std::env;

pub struct OtelGuard {
    pub logger_provider: global::GlobalLoggerProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        global::shutdown_logger_provider();
    }
}

pub fn init() -> color_eyre::Result<OtelGuard> {
    let _logger = logger::init();

    let guard = OtelGuard {
        logger_provider: global::logger_provider(),
    };

    Ok(guard)
}

/// Create a Resource that captures information about the entity for which telemetry is recorded.
fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
            KeyValue::new(
                DEPLOYMENT_ENVIRONMENT,
                env::var("DEPLOYMENT_ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            ),
        ],
        SCHEMA_URL,
    )
}
