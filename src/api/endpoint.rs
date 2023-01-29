use std::borrow::Cow;

use async_trait::async_trait;
use http::Uri;
use http::{HeaderMap, Method, Request};
use serde::de::DeserializeOwned;
use url::Url;

use super::error::BodyError;
use super::{client::Client, error::ApiError, params::QueryParams, query::Query};
use super::{AsyncClient, AsyncQuery};

pub trait Endpoint {
    fn url(&self) -> Cow<'static, str>;
    fn method(&self) -> Method;
    fn body(&self) -> Result<Option<Vec<u8>>, BodyError> {
        Ok(None)
    }
    fn headers(&self) -> Option<HeaderMap> {
        None
    }
    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    C: Client,
    T: DeserializeOwned,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.url())?;
        self.parameters().add_to_url(&mut url);
        let mut request = Request::builder()
            .method(self.method())
            .uri(url_to_http_uri(url))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");

        if let Some(headers) = self.headers() {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }

        let body = self.body()?.unwrap_or_default();

        let response = client.rest(request, body)?;
        let status = response.status();
        let value = if let Ok(val) = serde_json::from_slice(response.body()) {
            val
        } else {
            return Err(ApiError::server_error(status, response.body()));
        };

        if !status.is_success() {
            return Err(ApiError::from_teamdeck(value));
        }

        serde_json::from_value::<T>(value).map_err(ApiError::data_type::<T>)
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<T, C> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.url())?;
        self.parameters().add_to_url(&mut url);
        let mut request = Request::builder()
            .method(self.method())
            .uri(url_to_http_uri(url))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");

        if let Some(headers) = self.headers() {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }

        let body = self.body()?.unwrap_or_default();

        let response = client.rest_async(request, body).await?;
        let status = response.status();
        let value = if let Ok(val) = serde_json::from_slice(response.body()) {
            val
        } else {
            return Err(ApiError::server_error(status, response.body()));
        };

        if !status.is_success() {
            return Err(ApiError::from_teamdeck(value));
        }

        serde_json::from_value::<T>(value).map_err(ApiError::data_type::<T>)
    }
}

pub fn url_to_http_uri(url: Url) -> Uri {
    url.as_str()
        .parse::<Uri>()
        .expect("failed to parse a url::Url as an http::Uri")
}
