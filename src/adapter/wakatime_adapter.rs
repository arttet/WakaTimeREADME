use crate::port::DataSourcePort;

use crate::application::config::WakaTimeConfig;
use crate::domain::entity::Metrics;

use reqwest::header::AUTHORIZATION;
use reqwest_middleware::ClientWithMiddleware;

use async_trait::async_trait;
use color_eyre::eyre::{eyre, Result};

#[derive(Debug)]
pub struct WakaTimeAdapter {
    cfg: WakaTimeConfig,
    client: ClientWithMiddleware,
}

impl WakaTimeAdapter {
    pub fn new(cfg: WakaTimeConfig, client: ClientWithMiddleware) -> Self {
        Self { cfg, client }
    }

    pub async fn fetch_data(&self) -> Result<String> {
        let url = url::Url::parse(&self.cfg.base_url)?
            .join("users/current/stats/")?
            .join(&self.cfg.time_range.to_string())?
            .to_string();

        let request = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Basic {}", &self.cfg.api_key));

        let response = request.send().await?;

        if response.status().is_success() {
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(eyre!("Failed to fetch data from WakaTime API: {}", &url))
        }
    }
}

#[async_trait]
impl DataSourcePort for WakaTimeAdapter {
    async fn get_metrics(&self) -> Result<Metrics> {
        let json_data = self.fetch_data().await?;
        let metrics: Metrics =
            serde_json::from_str(&json_data).map_err(|err| eyre!("Error parsing JSON: {}", err))?;

        Ok(metrics)
    }
}
