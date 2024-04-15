use tracing::debug;

use crate::{error::SafeError, redis};

#[derive(Default)]
pub struct Request {
    url: Option<String>,
    method: Option<reqwest::Method>,
    cache: Option<bool>,
}

impl Request {
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn with_method(mut self, method: reqwest::Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn with_cache(mut self, value: bool) -> Self {
        self.cache = Some(value);
        self
    }

    pub async fn text(self) -> Result<String, SafeError> {
        let key = format!(
            "request:{}:{}",
            self.method.as_ref().unwrap(),
            self.url.as_ref().unwrap()
        );

        let cache = self.cache.unwrap_or(false);

        if cache {
            debug!("cache hit for: {key}");
            if let Some(value) = redis::get(&key).await? {
                return Ok(value);
            }
        } else {
            debug!("cache miss for: {key}");
        }

        let req = reqwest::Client::new()
            .request(
                self.method.as_ref().unwrap().clone(),
                self.url.as_ref().unwrap(),
            )
            .send()
            .await?;

        let text = req.text().await?;

        if cache {
            redis::set(&key, &text).await?;
        }

        return Ok(text);
    }
}

pub fn request() -> Request {
    Request::default()
}
