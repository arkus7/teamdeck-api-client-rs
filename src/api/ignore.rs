use async_trait::async_trait;
use http::Request;

use crate::api::{ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Query};

use super::endpoint;

/// A query modifier that ignores the data returned from an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ignore<E> {
    endpoint: E,
}

/// Ignore the resulting data from an endpoint.
pub fn ignore<E>(endpoint: E) -> Ignore<E> {
    Ignore { endpoint }
}

impl<E, C> Query<(), C> for Ignore<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint.url())?;
        self.endpoint.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.endpoint.method())
            .uri(endpoint::url_to_http_uri(url));
        let data = self.endpoint.body().unwrap_or_default();
        let rsp = client.rest(req, data)?;
        if !rsp.status().is_success() {
            let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(rsp.status(), rsp.body()));
            };
            return Err(ApiError::from_teamdeck(v));
        }

        Ok(())
    }
}

#[async_trait]
impl<E, C> AsyncQuery<(), C> for Ignore<E>
where
    E: Endpoint + Sync,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint.url())?;
        self.endpoint.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.endpoint.method())
            .uri(endpoint::url_to_http_uri(url));
        let data = self.endpoint.body().unwrap_or_default();
        let rsp = client.rest_async(req, data).await?;
        if !rsp.status().is_success() {
            let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(rsp.status(), rsp.body()));
            };
            return Err(ApiError::from_teamdeck(v));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use http::{Method, StatusCode};
    use serde_json::json;

    use crate::api::endpoint::Endpoint;
    use crate::api::{self, ApiError, AsyncQuery, Query};
    use crate::test::client_v2::{ExpectedRequest, TestClient};

    struct Dummy;

    impl Endpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn url(&self) -> Cow<'static, str> {
            "dummy".into()
        }
    }

    #[derive(Debug)]
    struct DummyResult {
        #[allow(dead_code)]
        value: u8,
    }

    #[test]
    fn test_non_json_response() {
        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/dummy")
            .response_body("not json")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        api::ignore(Dummy).query(&client).unwrap()
    }

    #[tokio::test]
    #[ignore = "Throws error 'Cannot drop a runtime in a context where blocking is not allowed. This happens when a runtime is dropped from within an asynchronous context.'"]
    async fn test_non_json_response_async() {
        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/dummy")
            .response_body("not json")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        api::ignore(Dummy).query_async(&client).await.unwrap()
    }

    #[test]
    fn test_teamdeck_error_bad_json() {
        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/dummy")
            .response_status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        let err = api::ignore(Dummy).query(&client).unwrap_err();
        if let ApiError::TeamdeckService { status, .. } = err {
            assert_eq!(status, http::StatusCode::NOT_FOUND);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_teamdeck_error_detection() {
        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/dummy")
            .response_status(StatusCode::NOT_FOUND)
            .response_body(
                json!({
                    "message": "dummy error message",
                })
                .to_string(),
            )
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        let err = api::ignore(Dummy).query(&client).unwrap_err();
        if let ApiError::Teamdeck { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_teamdeck_error_detection_legacy() {
        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/dummy")
            .response_status(StatusCode::NOT_FOUND)
            .response_body(
                json!({
                    "error": "dummy error message",
                })
                .to_string(),
            )
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        let err = api::ignore(Dummy).query(&client).unwrap_err();
        if let ApiError::Teamdeck { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_teamdeck_error_detection_unknown() {
        let err_obj = json!({
            "bogus": "dummy error message",
        });
        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/dummy")
            .response_status(StatusCode::NOT_FOUND)
            .response_body(err_obj.to_string())
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        let err = api::ignore(Dummy).query(&client).unwrap_err();
        if let ApiError::TeamdeckUnrecognized { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }
}
