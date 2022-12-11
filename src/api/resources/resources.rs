use crate::api::{paged::Pageable, params::ParamValue, sort_by::SortBy, Endpoint, QueryParams};
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
    sort: Option<SortBy<ResourcesSortBy>>,
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
    use http::Method;

    use super::{Resources, ResourcesExpand};
    use crate::api::query::Query;
    use crate::api::resources::resources::ResourcesSortBy;
    use crate::api::sort_by::SortBy;
    use crate::api::{self};
    use crate::test::client_v2::{ExpectedRequest, TestClient};

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
        let endpoint = api::ignore(Resources::builder().build().unwrap());

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/resources")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort_ascending() {
        let endpoint = api::ignore(
            Resources::builder()
                .sort(SortBy::Asc(ResourcesSortBy::Email))
                .build()
                .unwrap(),
        );

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/resources")
            .query("sort=email")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort_descending() {
        let endpoint = api::ignore(
            Resources::builder()
                .sort(SortBy::Desc(ResourcesSortBy::Email))
                .build()
                .unwrap(),
        );

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/resources")
            .query("sort=-email")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_active_true() {
        let endpoint = api::ignore(Resources::builder().active(true).build().unwrap());

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/resources")
            .query("active=1")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_active_default() {
        let endpoint = api::ignore(Resources::builder().build().unwrap());

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/resources")
            .query("")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_active_false() {
        let endpoint = api::ignore(Resources::builder().active(false).build().unwrap());

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/resources")
            .query("active=0")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_page() {
        let endpoint = api::ignore(Resources::builder().page(2).build().unwrap());

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/resources")
            .query("page=2")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = api::ignore(Resources::builder().name("test").build().unwrap());

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/resources")
            .query("name=test")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn endpoint_email() {
        let endpoint = api::ignore(Resources::builder().email("test@test.com").build().unwrap());
        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/resources")
            .query("email=test%40test.com")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();

        client.assert_expectations();
    }

    #[test]
    fn endpoint_expand() {
        let endpoint = api::ignore(
            Resources::builder()
                .expand(ResourcesExpand::CustomFieldValues)
                .build()
                .unwrap(),
        );

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/resources")
            .query("expand=custom_field_values")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();

        client.assert_expectations();
    }
}
