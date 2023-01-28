use async_trait::async_trait;
use http::Request;
use serde::de::DeserializeOwned;

use super::{
    endpoint::url_to_http_uri, ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Query,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pagination {
    All,
}

impl Default for Pagination {
    fn default() -> Self {
        Self::All
    }
}

const MAX_PAGE_SIZE: usize = 10;

impl Pagination {
    pub(crate) fn page_limit(self) -> usize {
        match self {
            Pagination::All => MAX_PAGE_SIZE,
        }
    }

    pub(crate) fn is_last_page(self, last_page_size: usize) -> bool {
        // If the last page didn't return any results, we're done.
        if last_page_size == 0 {
            return true;
        }

        // If the last page has fewer elements than our limit, we're definitely done.
        if last_page_size < self.page_limit() {
            return true;
        }

        // We're not done yet.
        false
    }
}

pub struct Paged<E> {
    endpoint: E,
    pagination: Pagination,
}

pub fn paged<E>(endpoint: E, pagination: Pagination) -> Paged<E> {
    Paged {
        endpoint,
        pagination,
    }
}

pub trait Pageable {}

impl<E, T, C> Query<Vec<T>, C> for Paged<E>
where
    E: Endpoint,
    E: Pageable,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, super::ApiError<<C>::Error>> {
        let mut page_num = 0;
        let mut results = vec![];

        let url = {
            let mut url = client.rest_endpoint(&self.endpoint.url())?;
            self.endpoint.parameters().add_to_url(&mut url);
            url
        };

        loop {
            let page_url = {
                let page_str = format!("{}", page_num);
                let mut page_url = url.clone();

                {
                    let mut pairs = page_url.query_pairs_mut();
                    pairs.append_pair("page", &page_str);
                }

                page_url
            };

            let request = Request::builder()
                .method(self.endpoint.method())
                .uri(url_to_http_uri(page_url))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json");

            let body = self.endpoint.body()?.unwrap_or_default();

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

            let page =
                serde_json::from_value::<Vec<T>>(value).map_err(ApiError::data_type::<Vec<T>>)?;
            let page_len = page.len();

            let is_last_page = {
                results.extend(page);
                self.pagination.is_last_page(page_len)
            };

            if is_last_page {
                break;
            }

            page_num += 1;
        }

        Ok(results)
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<Vec<T>, C> for Paged<E>
where
    E: Endpoint + Sync,
    E: Pageable,
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let mut page_num = 1;
        let mut results = vec![];

        let url = {
            let mut url = client.rest_endpoint(&self.endpoint.url())?;
            self.endpoint.parameters().add_to_url(&mut url);
            url
        };

        loop {
            let page_url = {
                let page_str = format!("{}", page_num);
                let mut page_url = url.clone();

                {
                    let mut pairs = page_url.query_pairs_mut();
                    pairs.append_pair("page", &page_str);
                }

                page_url
            };

            let request = Request::builder()
                .method(self.endpoint.method())
                .uri(url_to_http_uri(page_url))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json");

            let body = self.endpoint.body()?.unwrap_or_default();

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

            let page =
                serde_json::from_value::<Vec<T>>(value).map_err(ApiError::data_type::<Vec<T>>)?;
            let page_len = page.len();

            let is_last_page = {
                results.extend(page);
                self.pagination.is_last_page(page_len)
            };

            if is_last_page {
                break;
            }

            page_num += 1;
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use http::{Method, StatusCode};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::borrow::Cow;

    use crate::{
        api::{self, ApiError, AsyncQuery, Endpoint, Query},
        test::client::{ExpectedRequest, TestClient},
    };

    use super::{Pageable, Pagination};

    #[derive(Debug, Default)]
    struct Dummy;

    impl Endpoint for Dummy {
        fn url(&self) -> Cow<'static, str> {
            "paged_dummy".into()
        }

        fn method(&self) -> Method {
            Method::GET
        }
    }

    impl Pageable for Dummy {}

    #[derive(Debug, Deserialize, Serialize)]
    struct DummyResult {
        value: u8,
    }

    #[test]
    fn test_teamdeck_non_json_response() {
        let endpoint = api::paged(Dummy::default(), Pagination::All);

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/paged_dummy")
            .query(vec![("page".into(), "0".into())])
            .response_body("not json")
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        let res: Result<Vec<DummyResult>, _> = endpoint.query(&client);

        mock.assert();

        let err = res.unwrap_err();
        if let ApiError::TeamdeckService { status, .. } = err {
            assert_eq!(status, StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_teamdeck_error_bad_json() {
        let endpoint = Dummy::default();

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/paged_dummy")
            .query(vec![("page".into(), "0".into())])
            .response_status(StatusCode::NOT_FOUND)
            .response_body("")
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All).query(&client);

        mock.assert();

        let err = res.unwrap_err();
        if let ApiError::TeamdeckService { status, .. } = err {
            assert_eq!(status, http::StatusCode::NOT_FOUND);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_teamdeck_error_detection() {
        let endpoint = Dummy::default();

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/paged_dummy")
            .query(vec![("page".into(), "0".into())])
            .response_status(StatusCode::NOT_FOUND)
            .response_body(
                json!({
                  "message": "dummy error message",
                })
                .to_string(),
            )
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All).query(&client);

        mock.assert();

        let err = res.unwrap_err();
        if let ApiError::Teamdeck { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_teamdeck_error_detection_legacy() {
        let endpoint = Dummy::default();

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/paged_dummy")
            .query(vec![("page".into(), "0".into())])
            .response_status(StatusCode::NOT_FOUND)
            .response_body(
                json!({
                  "error": "dummy error message",
                })
                .to_string(),
            )
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All).query(&client);

        mock.assert();

        let err = res.unwrap_err();
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

        let endpoint = Dummy::default();

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/paged_dummy")
            .query(vec![("page".into(), "0".into())])
            .response_status(StatusCode::NOT_FOUND)
            .response_body(err_obj.to_string())
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All).query(&client);

        mock.assert();

        let err = res.unwrap_err();
        if let ApiError::TeamdeckUnrecognized { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_pagination_all() {
        let results = (0..=255)
            .map(|value| DummyResult { value })
            .collect::<Vec<_>>();

        let pages: Vec<_> = results.chunks(10).collect();

        assert_eq!(pages.len(), 26);

        let expected_requests = pages.iter().enumerate().map(|(i, page)| {
            ExpectedRequest::builder()
                .method(Method::GET)
                .path("/paged_dummy")
                .query(vec![("page".into(), i.to_string())])
                .response_body(json!(page).to_string())
                .build()
                .unwrap()
        });

        let client = TestClient::new();

        let mocks = expected_requests
            .map(|expected| client.expect(expected))
            .collect::<Vec<_>>();

        let query = Dummy::default();

        let res: Vec<DummyResult> = api::paged(query, Pagination::All).query(&client).unwrap();

        for mock in mocks {
            mock.assert();
        }

        assert_eq!(res.len(), 256);

        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[tokio::test]
    #[ignore = "Throws error 'Cannot drop a runtime in a context where blocking is not allowed. This happens when a runtime is dropped from within an asynchronous context.'"]
    async fn test_pagination_all_async() {
        let results = (0..=255)
            .map(|value| DummyResult { value })
            .collect::<Vec<_>>();

        let pages: Vec<_> = results.chunks(10).collect();

        assert_eq!(pages.len(), 26);

        let expected_requests = pages.iter().enumerate().map(|(i, page)| {
            ExpectedRequest::builder()
                .method(Method::GET)
                .path("/paged_dummy")
                .query(vec![("page".into(), i.to_string())])
                .response_body(json!(page).to_string())
                .build()
                .unwrap()
        });

        let client = TestClient::new();

        let mocks = expected_requests
            .map(|expected| client.expect(expected))
            .collect::<Vec<_>>();

        let query = Dummy::default();

        let res: Vec<DummyResult> = api::paged(query, Pagination::All)
            .query_async(&client)
            .await
            .unwrap();

        for mock in mocks {
            mock.assert();
        }

        assert_eq!(res.len(), 256);

        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }
}
