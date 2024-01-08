use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use task_local_extensions::Extensions;

use log::debug;

use std::time::Instant;

pub struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let url = req.url().clone();
        let path = url.path();

        let start = Instant::now();
        let res = next.run(req, extensions).await;
        let elapsed = start.elapsed();

        debug!("Request to '{}' took '{}'ms", path, elapsed.as_millis());

        res
    }
}
