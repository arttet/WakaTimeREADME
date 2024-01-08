use log::Level;

use opentelemetry::global;

use opentelemetry_sdk::logs::{Config, LoggerProvider};

use opentelemetry_appender_log::OpenTelemetryLogBridge;

pub fn init() -> LoggerProvider {
    // Setup LoggerProvider with a stdout exporter
    let exporter = opentelemetry_stdout::LogExporterBuilder::default()
        // .with_encoder(|writer, data| Ok(serde_json::to_writer_pretty(writer, &data).unwrap()))
        .build();

    let logger_provider = LoggerProvider::builder()
        .with_config(Config::default().with_resource(super::resource()))
        .with_simple_exporter(exporter)
        .build();

    global::set_logger_provider(logger_provider.clone());

    // Setup Log Appender for the log crate.
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    log::set_max_level(Level::Debug.to_level_filter());

    logger_provider
}
