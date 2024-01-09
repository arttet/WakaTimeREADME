use super::entity::Language;

use crate::port::{DataSourcePort, UploadContentPort};

use crate::application::config::{ContentConfig, TimeRange};

use chrono::{Datelike, Duration, Utc};
use color_eyre::eyre::Result;
use log::debug;

use std::fmt::Write as _;

/// ContentManager processes metrics received from WakaTime.
pub struct ContentManager<TDataPort, TContentUploader>
where
    TDataPort: DataSourcePort,
    TContentUploader: UploadContentPort,
{
    cfg: ContentConfig,
    data_port: TDataPort,
    content_uploader: TContentUploader,
}

impl<TDataPort, TContentUploader> ContentManager<TDataPort, TContentUploader>
where
    TDataPort: DataSourcePort,
    TContentUploader: UploadContentPort,
{
    /// Creates a new instance of ContentManager.
    pub fn new(
        cfg: ContentConfig,
        data_port: TDataPort,
        content_uploader: TContentUploader,
    ) -> Self {
        ContentManager {
            cfg,
            data_port,
            content_uploader,
        }
    }

    /// Processes metrics from WakaTime and generate a report.
    ///
    /// # Returns
    ///
    /// * `Ok(())`: If metrics are successfully processed.
    /// * `Err(...)`: If an error occurs while processing metrics.
    pub async fn process_metrics(&self) -> Result<()> {
        let metrics = self.data_port.get_metrics().await?;
        let mut buf = String::with_capacity(1024);

        write!(&mut buf, "\n```text\n")?;

        if self.cfg.show_title {
            self.make_title(&mut buf, &metrics.data.range)?;
        }

        self.make_table(&mut buf, &metrics.data.languages)?;

        if self.cfg.show_total_time {
            self.make_footer(
                &mut buf,
                &metrics.data.human_readable_total_including_other_language,
            )?;
        }

        write!(&mut buf, "\n```\n")?;

        debug!("{}", &buf);

        self.content_uploader.upload_content(buf).await?;

        Ok(())
    }
}

impl<TDataPort, TContentUploader> ContentManager<TDataPort, TContentUploader>
where
    TDataPort: DataSourcePort,
    TContentUploader: UploadContentPort,
{
    fn make_title(&self, buf: &mut String, range: &str) -> Result<()> {
        let time_range = TimeRange::try_from(range)?;

        let end_date = Utc::now();
        let start_date = match time_range {
            TimeRange::Last7Days => end_date - Duration::days(7),
            TimeRange::Last30Days => end_date - Duration::days(30),
        };

        let week: u32 = start_date.date_naive().iso_week().week();

        write!(
            &mut *buf,
            "Week #{}: {} - {}\n\n",
            week,
            start_date.format(&self.cfg.date_style),
            end_date.format(&self.cfg.date_style)
        )?;

        Ok(())
    }

    fn make_table(&self, buf: &mut String, languages: &[Language]) -> Result<()> {
        for language in languages.iter().take(self.cfg.language_count) {
            if (language.total_seconds as u64) < self.cfg.threshold.as_secs() {
                continue;
            }

            let progress_bar = self.make_progress_bar(language.percent);

            writeln!(
                &mut *buf,
                "{:<10} {:>16} {} {:>6.2} %",
                language.name, language.text, progress_bar, language.percent,
            )?;
        }

        writeln!(&mut *buf)?;

        Ok(())
    }

    fn make_progress_bar(&self, percent: f64) -> String {
        let progress_bar_len = self.cfg.progress_bar_len;
        let block_style = &self.cfg.block_style;

        let mut progress_bar = String::with_capacity(progress_bar_len);

        let markers = block_style.chars().count() - 1;
        let proportion = percent / 100.0 * progress_bar_len as f64;
        let mut remaining_length = (proportion + 0.5 / markers as f64) as usize;

        for _ in 0..markers {
            progress_bar.push_str(
                &block_style
                    .chars()
                    .last()
                    .unwrap()
                    .to_string()
                    .repeat(remaining_length),
            );

            let remainder_block = ((proportion - progress_bar.chars().count() as f64)
                * markers as f64
                + 0.5) as usize;
            if remainder_block > 0 {
                progress_bar.push(block_style.chars().nth(remainder_block).unwrap());
            }

            remaining_length = (proportion - progress_bar.chars().count() as f64) as usize;
        }
        progress_bar.push_str(
            &block_style
                .chars()
                .next()
                .unwrap()
                .to_string()
                .repeat(progress_bar_len - progress_bar.chars().count()),
        );

        progress_bar
    }

    fn make_footer(&self, buf: &mut String, total_time: &str) -> Result<()> {
        write!(&mut *buf, "Total Time: {}", total_time,)?;

        Ok(())
    }
}
