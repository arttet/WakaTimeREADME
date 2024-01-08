use crate::domain::entity::Metrics;

use async_trait::async_trait;
use color_eyre::eyre::Result;

/// Port defining interactions with the WakaTime service.
#[async_trait]
pub trait DataSourcePort {
    /// Get metrics from the WakaTime service and returns them as a `WakaTimeMetrics` structure.
    ///
    /// # Returns
    ///
    /// * `Ok(WakaTimeMetrics)`: If the metrics are successfully downloaded and parsed.
    /// * `Err(eyre::Report)`: If there is an error during the download or parsing, providing an error report.
    async fn get_metrics(&self) -> Result<Metrics>;
}
