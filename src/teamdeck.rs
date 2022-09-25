use std::fmt;

use http::HeaderValue;
use http::Response as HttpResponse;
use log::{debug, error, info};
use reqwest::blocking::Client;
use reqwest::Client as AsyncClient;

use thiserror::Error;
use url::Url;

use crate::api;

#[derive(Clone)]
pub struct Teamdeck {
    client: Client,
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
        let client = Client::new();

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
            let request = dbg!(http_request.try_into()?);
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
