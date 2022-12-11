use derive_builder::Builder;
use std::borrow::Cow;

use crate::api::{Endpoint, QueryParams};

use super::many::ResourcesExpand;

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
        api::{self, resources::many::ResourcesExpand, Query},
        test::client::{ExpectedRequest, TestClient},
    };
    #[test]
    fn resource() {
        let endpoint = api::ignore(Resource::builder().id(1).build().unwrap());

        let expected = ExpectedRequest::builder()
            .path("/resources/1")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn resource_expand() {
        let endpoint = api::ignore(
            Resource::builder()
                .id(1)
                .expand(Some(ResourcesExpand::CustomFieldValues))
                .build()
                .unwrap(),
        );

        let expected = ExpectedRequest::builder()
            .path("/resources/1")
            .query("expand=custom_field_values")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        let _ = endpoint.query(&client);
    }
}
