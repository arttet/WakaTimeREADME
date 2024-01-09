use serde::{Deserialize, Serialize};

use color_eyre::eyre::{eyre, ErrReport, Result};
use duration_str::deserialize_duration;

use std::env;
use std::time::Duration;

const CONFIG_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config/config.toml"));

#[derive(Debug, Serialize, Deserialize)]
pub enum TimeRange {
    #[serde(rename = "last_7_days")]
    Last7Days,
    #[serde(rename = "last_30_days")]
    Last30Days,
}

impl ToString for TimeRange {
    fn to_string(&self) -> String {
        match self {
            TimeRange::Last7Days => String::from("last_7_days"),
            TimeRange::Last30Days => String::from("last_30_days"),
        }
    }
}

impl TryFrom<&str> for TimeRange {
    type Error = ErrReport;

    fn try_from(value: &str) -> Result<TimeRange> {
        match value {
            "last_7_days" => Ok(TimeRange::Last7Days),
            "last_30_days" => Ok(TimeRange::Last30Days),
            _ => Err(eyre!("Invalid Time Range")),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct HttpClientConfig {
    pub max_retries: u32,
    #[serde(deserialize_with = "deserialize_duration")]
    pub timeout: Duration,
}

#[derive(Debug, Deserialize)]
pub struct WakaTimeConfig {
    pub api_key: String,
    pub base_url: String,
    pub time_range: TimeRange,
}

#[derive(Debug, Deserialize)]
pub struct ContentConfig {
    pub date_style: String,
    pub block_style: String,
    pub progress_bar_len: usize,
    pub language_count: usize,
    #[serde(deserialize_with = "deserialize_duration")]
    pub threshold: Duration,

    pub show_title: bool,
    pub show_time: bool,
    pub show_total_time: bool,
}

#[cfg(feature = "git")]
#[derive(Debug, Deserialize)]
pub struct GitConfig {
    pub token: String,

    pub base_url: String,
    pub repository: String,
    pub branch_name: String,
    pub file_path: String,

    pub begin_block: String,
    pub end_block: String,

    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub commit_message: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub http_client: HttpClientConfig,
    pub wakatime: WakaTimeConfig,
    pub content: ContentConfig,
    #[cfg(feature = "git")]
    pub git: GitConfig,
}

impl Config {
    pub fn new() -> Self {
        // Load default configuration from a file
        let mut cfg: Config = toml::from_str(CONFIG_FILE).expect("Failed to parse TOML.");

        // Load required environment variables
        cfg.wakatime.api_key =
            env::var("WAKATIME_API_KEY").expect("Env variable `WAKATIME_API_KEY` should be set.");

        #[cfg(feature = "git")]
        {
            let git_config = git2::Config::open_default().expect(
                "Failed to open Git configuration. Make sure Git is installed and configured.",
            );

            cfg.git.token = env::var("GIT_TOKEN").expect("Env variable `GIT_TOKEN` should be set.");

            cfg.git.repository =
                env::var("GIT_REPOSITORY").expect("Env variable `GIT_REPOSITORY` should be set.");

            if let Ok(author_name) = env::var("GIT_AUTHOR_NAME") {
                cfg.git.author_name = author_name;
            } else {
                cfg.git.author_name = git_config
                    .get_string("user.name")
                    .expect("Failed to get author's name from Git configuration.");
            }

            if let Ok(author_email) = env::var("GIT_AUTHOR_EMAIL") {
                cfg.git.author_email = author_email;
            } else {
                cfg.git.author_email = git_config
                    .get_string("user.email")
                    .expect("Failed to get author's email from Git configuration.");
            }

            if let Ok(committer_name) = env::var("GIT_COMMITTER_NAME") {
                cfg.git.committer_name = committer_name;
            } else {
                cfg.git.committer_name = git_config
                    .get_string("user.name")
                    .expect("Failed to get committer's name from Git configuration.");
            }

            if let Ok(committer_email) = env::var("GIT_COMMITTER_EMAIL") {
                cfg.git.committer_email = committer_email;
            } else {
                cfg.git.committer_email = git_config
                    .get_string("user.email")
                    .expect("Failed to get committer's email from Git configuration.");
            }

            if let Ok(commit_message) = env::var("GIT_COMMIT_MESSAGE") {
                cfg.git.commit_message = commit_message;
            }
        }

        cfg
    }
}
