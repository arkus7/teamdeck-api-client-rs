use std::fmt;

use async_trait::async_trait;
use bytes::Bytes;
use http::HeaderValue;
use http::Response as HttpResponse;
use log::{debug, error, info};
use reqwest::blocking::Client as BlockingClient;
use reqwest::Client as AsyncClient;

use thiserror::Error;
use url::Url;

use crate::api;

#[derive(Clone)]
pub struct Teamdeck {
    client: BlockingClient,
    base_url: Url,
    token: String,
}

impl fmt::Debug for Teamdeck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Teamdeck")
            .field("base_url", &self.base_url)
            .finish()
    }
}

impl Teamdeck {
    pub fn new<T: Into<String>>(token: T) -> Self {
        let base_url = Url::parse("https://api.teamdeck.io/v1/").unwrap();
        let client = BlockingClient::new();

        Self {
            client,
            base_url,
            token: token.into(),
        }
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    #[error("error setting auth header")]
    AuthError,
    #[error("communication with gitlab: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("`http` error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}

impl api::RestClient for Teamdeck {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!(target: "teamdeck", "REST api call {}", endpoint);
        Ok(self.base_url.join(endpoint)?)
    }
}

impl api::Client for Teamdeck {
    fn rest(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, api::ApiError<Self::Error>> {
        let call = || -> Result<_, RestError> {
            if let Some(headers) = request.headers_mut() {
                let mut value =
                    HeaderValue::from_str(&self.token).map_err(|_| RestError::AuthError)?;
                value.set_sensitive(true);

                headers.insert("x-api-key", value);
            };

            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let response = self.client.execute(request)?;

            let mut http_response = HttpResponse::builder()
                .status(response.status())
                .version(response.version());
            let headers = http_response.headers_mut().unwrap();
            for (key, value) in response.headers() {
                headers.insert(key, value.clone());
            }

            Ok(http_response.body(response.bytes()?)?)
        };

        call().map_err(api::ApiError::client)
    }
}

#[derive(Clone)]
pub struct AsyncTeamdeck {
    client: AsyncClient,
    base_url: Url,
    token: String,
}

impl AsyncTeamdeck {
    pub fn new<T: Into<String>>(token: T) -> Self {
        let base_url = Url::parse("https://api.teamdeck.io/v1/").unwrap();
        let client = AsyncClient::new();

        Self {
            client,
            base_url,
            token: token.into(),
        }
    }
}

impl fmt::Debug for AsyncTeamdeck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AsyncTeamdeck")
            .field("base_url", &self.base_url)
            .finish()
    }
}

#[async_trait]
impl api::RestClient for AsyncTeamdeck {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!(target: "teamdeck", "REST api call {}", endpoint);
        Ok(self.base_url.join(endpoint)?)
    }
}

#[async_trait]
impl api::AsyncClient for AsyncTeamdeck {
    async fn rest_async(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            if let Some(headers) = request.headers_mut() {
                let mut value =
                    HeaderValue::from_str(&self.token).map_err(|_| RestError::AuthError)?;
                value.set_sensitive(true);

                headers.insert("x-api-key", value);
            };
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };
        call().map_err(api::ApiError::client).await
    }
}
