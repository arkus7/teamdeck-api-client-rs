use async_trait::async_trait;
use derive_builder::Builder;
use http::{Method, StatusCode};
use httpmock::{Mock, MockServer};
use reqwest::blocking::Client as BlockingClient;
use thiserror::Error;
use url::Url;

use crate::api::{self, ApiError, AsyncClient, Client, RestClient};

struct MockId(usize);

pub(crate) struct TestClient {
    server: MockServer,
    client: BlockingClient,
    mocks: Vec<MockId>,
}

impl TestClient {
    pub(crate) fn new() -> Self {
        let server = MockServer::start();
        let client = BlockingClient::new();
        let mocks = Vec::new();
        Self {
            server,
            client,
            mocks,
        }
    }

    pub(crate) fn expecting(req: ExpectedRequest) -> Self {
        let mut client = Self::new();
        client.expect(req);
        client
    }

    pub(crate) fn expect(&mut self, req: ExpectedRequest) -> Mock {
        let mock = mock_request(&self.server, req);
        self.mocks.push(MockId(mock.id));
        mock
    }

    pub(crate) fn url_str(&self, path: &str) -> String {
        self.server.url(&format!("/{}", path))
    }

    pub(crate) fn assert_mocks(&self) {
        for MockId(id) in &self.mocks {
            let mock = Mock::new(*id, &self.server);
            mock.assert();
        }
    }
}

fn mock_request<'a>(server: &'a MockServer, req: ExpectedRequest) -> Mock<'a> {
    let mock = server.mock(|when, then| {
        let mut when = when.method(req.method.as_str()).path(req.path);

        if let Some(body) = req.request_body {
            when = when.json_body(body);
        }

        if let Some(headers) = req.request_headers {
            for (key, value) in headers {
                when = when.header(key, value);
            }
        }

        if let Some(query) = req.query {
            for (key, value) in query {
                when = when.query_param(key, value);
            }
        }

        let mut then = then.status(req.response_status.as_u16());
        if let Some(headers) = req.response_headers {
            for (key, value) in headers {
                then = then.header(key, value);
            }
        }
        if let Some(body) = req.response_body {
            _ = then.body(body);
        }
    });

    mock
}

#[derive(Debug, Error)]
#[error("test client error")]
pub enum TestClientError {}

impl RestClient for TestClient {
    type Error = TestClientError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(Url::parse(&self.url_str(endpoint))?)
    }
}

impl Client for TestClient {
    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, ApiError<<Self as api::RestClient>::Error>> {
        let request = request.body(body).unwrap();
        let request = request.try_into().unwrap();
        let rsp = self.client.execute(request).unwrap();

        let mut http_rsp = http::Response::builder()
            .status(rsp.status())
            .version(rsp.version());

        let headers = http_rsp.headers_mut().unwrap();
        for (key, value) in rsp.headers() {
            headers.insert(key, value.clone());
        }

        let rsp_bytes = rsp.bytes().unwrap();

        Ok(http_rsp.body(rsp_bytes).unwrap())
    }
}

#[async_trait]
impl AsyncClient for TestClient {
    async fn rest_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, ApiError<<Self as RestClient>::Error>> {
        <Self as Client>::rest(self, request, body)
    }
}

impl Drop for TestClient {
    fn drop(&mut self) {
      self.assert_mocks();
    }
}

#[derive(Builder, Debug)]
pub(crate) struct ExpectedRequest {
    #[builder(default = "Method::GET")]
    method: Method,
    path: &'static str,
    #[builder(default = "StatusCode::OK")]
    response_status: StatusCode,
    #[builder(default, setter(strip_option))]
    query: Option<Vec<(String, String)>>,
    #[builder(default, setter(strip_option))]
    request_headers: Option<Vec<(String, String)>>,
    #[builder(default, setter(strip_option, into))]
    request_body: Option<serde_json::Value>,
    #[builder(default, setter(strip_option, into))]
    response_body: Option<String>,
    #[builder(default, setter(strip_option))]
    response_headers: Option<Vec<(String, String)>>,
}

impl ExpectedRequest {
    pub(crate) fn builder() -> ExpectedRequestBuilder {
        ExpectedRequestBuilder::default()
    }
}
