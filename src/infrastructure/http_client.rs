use crate::application::config::HttpClientConfig;
use crate::infrastructure::middleware::LoggingMiddleware;

use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub struct HttpClientBuilder {
    cfg: HttpClientConfig,
}

impl HttpClientBuilder {
    pub fn new(cfg: HttpClientConfig) -> Self {
        Self { cfg }
    }

    pub fn build(&self) -> ClientWithMiddleware {
        let retry_policy =
            ExponentialBackoff::builder().build_with_max_retries(self.cfg.max_retries);

        let client = Client::builder()
            .user_agent(APP_USER_AGENT)
            .timeout(self.cfg.timeout.into())
            .build()
            .unwrap();

        let client = ClientBuilder::new(client)
            .with(LoggingMiddleware)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        client
    }
}
