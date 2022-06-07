use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use http::request::Builder as RequestBuilder;
use http::Response;
use url::Url;

use super::error::ApiError;

pub trait RestClient {
    type Error: Error + Send + Sync + 'static;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;
}

/// A trait representing a client which can communicate with a GitLab instance.
pub trait Client: RestClient {
    /// Send a REST query.
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing an asynchronous client which can communicate with a GitLab instance.
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously.
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}
