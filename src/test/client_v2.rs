use async_trait::async_trait;
use httptest::{Expectation, ServerHandle, ServerPool};
use reqwest::blocking::Client as BlockingClient;
use thiserror::Error;
use url::Url;
use derive_builder::Builder;

pub use httptest::matchers;
pub use httptest::responders;

// Create a server pool that will create at most 2 servers.
static SERVER_POOL: ServerPool = ServerPool::new(3);

use crate::api::{self, ApiError, AsyncClient, Client, RestClient};
pub(crate) struct TestClient<'a> {
    server: ServerHandle<'a>,
    client: BlockingClient,
}

impl<'a> TestClient<'a> {
    pub(crate) fn new() -> Self {
        let server = SERVER_POOL.get_server();
        let client = BlockingClient::new();
        Self { server, client }
    }

    pub(crate) fn expect(&self, expectation: Expectation) -> &Self {
        self.server.expect(expectation);
        self
    }

    pub(crate) fn expect_request(&self, req: ExpectedRequest) -> &Self {
      self.server.expect(req.into());
      self
  }

    pub(crate) fn url_str(&self, path: &str) -> String {
      self.server.url_str(&format!("/{}", path))
    }

    pub(crate) fn assert_expectations(mut self) {
        self.server.verify_and_clear();
    }
}

#[derive(Builder, Debug)]
pub(crate) struct ExpectedRequest{
    method: &'static str,
    path: &'static str,
    respond_with_status: u16,
    #[builder(default, setter(strip_option, into))]
    query: Option<&'static str>,
    // #[builder(default, setter(strip_option, into))]
    // request_headers: Option<Vec<(String, String)>>,
    #[builder(default, setter(strip_option, into))]
    request_body: Option<serde_json::Value>,
    #[builder(default, setter(strip_option, into))]
    response_body: Option<serde_json::Value>,
    #[builder(default, setter(strip_option, into))]
    response_headers: Option<Vec<(String, String)>>,
    #[builder(default, setter(strip_option, into))]
    times: Option<usize>
}

impl ExpectedRequest {
  pub(crate) fn builder() -> ExpectedRequestBuilder {
    ExpectedRequestBuilder::default()
  }
}

impl From<ExpectedRequest> for Expectation {
    fn from(expect: ExpectedRequest) -> Self {
        let method_matcher = matchers::request::method(expect.method);
        let path_matcher = matchers::request::path(expect.path);

        let query_matcher = matchers::request::query(expect.query.unwrap_or_default());
        let request_body_matcher = matchers::request::body(
            expect
                .request_body
                .map(|body| serde_json::to_string(&body).unwrap())
                .unwrap_or_default()
        );

        let matcher = matchers::all_of![
            method_matcher,
            path_matcher,
            query_matcher,
            request_body_matcher
        ];

        let response_headers = expect.response_headers.unwrap_or_default();
        
        let mut responder = responders::status_code(expect.respond_with_status);
        for (key, value) in response_headers {
            responder = responder.append_header(key, value);
        }
        if let Some(body) = expect.response_body {
          responder = responder.body(Box::leak(body.to_string().into_boxed_str()));
        }

        let exectation = Expectation::matching(matcher);
        if let Some(times) = expect.times {
            exectation.times(times).respond_with(responder)
        } else {
            exectation.respond_with(responder)
        }
    }
}

#[derive(Debug, Error)]
#[error("test client error")]
pub enum TestClientError {}

impl<'a> RestClient for TestClient<'a> {
    type Error = TestClientError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(Url::parse(&self.url_str(endpoint))?)
    }
}

impl<'a> Client for TestClient<'a> {
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

        Ok(http_rsp.body(rsp.bytes().unwrap()).unwrap())
    }
}

#[async_trait]
impl<'a> AsyncClient for TestClient<'a> {
    async fn rest_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, ApiError<<Self as RestClient>::Error>> {
        <Self as Client>::rest(self, request, body)
    }
}