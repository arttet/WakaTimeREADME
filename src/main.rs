mod adapter;
mod application;
mod domain;
mod infrastructure;
mod port;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    dotenv::dotenv().ok();

    let cfg = application::config::Config::new();

    #[cfg(feature = "telemetry")]
    let _guard = infrastructure::telemetry::init().ok();

    let client = infrastructure::HttpClientBuilder::new(cfg.http_client).build();

    let data_port = adapter::WakaTimeAdapter::new(cfg.wakatime, client);
    let content_uploader = adapter::GitAdapter::new(cfg.git);
    let content_manager = domain::ContentManager::new(cfg.content, data_port, content_uploader);

    content_manager.process_metrics().await?;

    Ok(())
}
