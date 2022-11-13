use std::borrow::Cow;

use derive_builder::Builder;

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
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn resource() {
        let client = SingleTestClient::new_raw(
            ExpectedUrl::builder()
                .endpoint("resources/1")
                .build()
                .unwrap(),
            "",
        );
        let endpoint = api::ignore(Resource::builder().id(1).build().unwrap());
        endpoint.query(&client).unwrap();
    }

    #[test]
    fn resource_expand() {
        let client = SingleTestClient::new_raw(
            ExpectedUrl::builder()
                .endpoint("resources/1")
                .add_query_params(&[("expand", "custom_field_values")])
                .build()
                .unwrap(),
            "",
        );
        let endpoint = Resource::builder()
            .id(1)
            .expand(Some(ResourcesExpand::CustomFieldValues))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
