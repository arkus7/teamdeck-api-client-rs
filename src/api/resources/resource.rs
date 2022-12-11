use derive_builder::Builder;
use std::borrow::Cow;

use crate::api::{Endpoint, QueryParams};

use super::resources::ResourcesExpand;

#[derive(Debug, Builder)]
pub struct Resource {
    id: usize,
    #[builder(default)]
    expand: Option<ResourcesExpand>,
}

impl Resource {
    pub fn builder() -> ResourceBuilder {
        ResourceBuilder::default()
    }
}

impl Endpoint for Resource {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn url(&self) -> Cow<'static, str> {
        format!("resources/{}", self.id).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("expand", self.expand);

        params
    }
}

#[cfg(test)]
mod tests {
    use super::Resource;
    use crate::{
        api::{self, resources::resources::ResourcesExpand, Query},
        test::{
            client_v2::{TestClient, ExpectedRequest},
        },
    };
    #[test]
    fn resource() {
        let client = TestClient::new();
        let endpoint = api::ignore(Resource::builder().id(1).build().unwrap());

        let expected = ExpectedRequest::builder()
            .method("GET")
            .path("/resources/1")
            .respond_with_status(200)
            .build()
            .unwrap();

        client.expect_request(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn resource_expand() {
        let client = TestClient::new();
        let endpoint = api::ignore(Resource::builder().id(1)
        .expand(Some(ResourcesExpand::CustomFieldValues)).build().unwrap());

        let expected = ExpectedRequest::builder()
            .method("GET")
            .path("/resources/1")
            .query("expand=custom_field_values")
            .respond_with_status(200)
            .build()
            .unwrap();
            
        client.expect_request(expected);

        let _ = endpoint.query(&client);

        client.assert_expectations();
    }
}
