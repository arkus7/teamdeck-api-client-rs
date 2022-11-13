use crate::api::{paged::Pageable, params::ParamValue, Endpoint, QueryParams};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourcesSortBy {
    Name,
    Email,
    Role,
    Active,
}

impl Default for ResourcesSortBy {
    fn default() -> Self {
        Self::Email
    }
}

impl ResourcesSortBy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Email => "email",
            Self::Role => "role",
            Self::Active => "active",
        }
    }
}

impl ParamValue<'static> for ResourcesSortBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResourcesExpand {
    CustomFieldValues,
}

impl ResourcesExpand {
    fn as_str(self) -> &'static str {
        match self {
            Self::CustomFieldValues => "custom_field_values",
        }
    }
}

impl ParamValue<'static> for ResourcesExpand {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Resources<'a> {
    #[builder(default)]
    sort: Option<ResourcesSortBy>,
    #[builder(default)]
    page: Option<u64>,
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    #[builder(default)]
    active: Option<bool>,
    #[builder(setter(into), default)]
    email: Option<Cow<'a, str>>,
    #[builder(default)]
    expand: Option<ResourcesExpand>,
}

impl<'a> Resources<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ResourcesBuilder<'a> {
        ResourcesBuilder::default()
    }
}

impl<'a> Endpoint for Resources<'a> {
    fn url(&self) -> Cow<'static, str> {
        "resources".into()
    }

    fn method(&self) -> http::Method {
        Method::GET
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("sort", self.sort)
            .push_opt("page", self.page)
            .push_opt("name", self.name.as_ref())
            .push_opt("active", self.active)
            .push_opt("email", self.email.as_ref())
            .push_opt("expand", self.expand);

        params
    }
}
impl<'a> Pageable for Resources<'a> {}

#[cfg(test)]
mod tests {
    use super::{Resources, ResourcesExpand};
    use crate::api::query::Query;
    use crate::api::resources::resources::ResourcesSortBy;
    use crate::{
        api::{self},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn sort_by_default() {
        assert_eq!(ResourcesSortBy::default(), ResourcesSortBy::Email);
    }

    #[test]
    fn order_by_as_str() {
        let items = &[
            (ResourcesSortBy::Active, "active"),
            (ResourcesSortBy::Email, "email"),
            (ResourcesSortBy::Name, "name"),
            (ResourcesSortBy::Role, "role"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn defaults_are_sufficient() {
        Resources::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("resources")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = api::ignore(Resources::builder().build().unwrap());
        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("resources")
            .add_query_params(&[("sort", "email")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = api::ignore(
            Resources::builder()
                .sort(ResourcesSortBy::Email)
                .build()
                .unwrap(),
        );
        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_active_true() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("resources")
            .add_query_params(&[("active", "1")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = api::ignore(Resources::builder().active(true).build().unwrap());
        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_active_default() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("resources")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = api::ignore(Resources::builder().build().unwrap());
        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_active_false() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("resources")
            .add_query_params(&[("active", "0")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = api::ignore(Resources::builder().active(false).build().unwrap());
        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_page() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("resources")
            .add_query_params(&[("page", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = api::ignore(Resources::builder().page(2).build().unwrap());
        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("resources")
            .add_query_params(&[("name", "test")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = api::ignore(Resources::builder().name("test").build().unwrap());
        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_email() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("resources")
            .add_query_params(&[("email", "test@test.com")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = api::ignore(Resources::builder().email("test@test.com").build().unwrap());
        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_expand() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("resources")
            .add_query_params(&[("expand", "custom_field_values")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = api::ignore(
            Resources::builder()
                .expand(ResourcesExpand::CustomFieldValues)
                .build()
                .unwrap(),
        );
        endpoint.query(&client).unwrap();
    }
}
