use crate::api::{paged::Pageable, params::ParamValue, sort_by::SortBy, Endpoint, QueryParams};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VacationReasonsSortBy {
    Name,
    Archived,
    Payable,
    Color,
    Id,
}

impl Default for VacationReasonsSortBy {
    fn default() -> Self {
        Self::Name
    }
}

impl VacationReasonsSortBy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Archived => "archived",
            Self::Payable => "payable",
            Self::Color => "color",
            Self::Id => "id",
        }
    }
}

impl ParamValue<'static> for VacationReasonsSortBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct VacationReasons<'a> {
    #[builder(default)]
    sort: Option<SortBy<VacationReasonsSortBy>>,
    #[builder(default)]
    page: Option<u64>,
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    #[builder(default)]
    archived: Option<bool>,
}

impl<'a> VacationReasons<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> VacationReasonsBuilder<'a> {
        VacationReasonsBuilder::default()
    }
}

impl<'a> Endpoint for VacationReasons<'a> {
    fn url(&self) -> Cow<'static, str> {
        "vacation-reasons".into()
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
            .push_opt("archived", self.archived);

        params
    }
}
impl<'a> Pageable for VacationReasons<'a> {}

#[cfg(test)]
mod tests {
    use http::Method;

    use super::*;
    use crate::api::query::Query;
    use crate::api::sort_by::SortBy;
    use crate::api::{self};
    use crate::test::client::{ExpectedRequest, TestClient};

    #[test]
    fn sort_by_default() {
        assert_eq!(
            VacationReasonsSortBy::default(),
            VacationReasonsSortBy::Name
        );
    }

    #[test]
    fn order_by_as_str() {
        let items = &[
            (VacationReasonsSortBy::Archived, "archived"),
            (VacationReasonsSortBy::Color, "color"),
            (VacationReasonsSortBy::Name, "name"),
            (VacationReasonsSortBy::Payable, "payable"),
            (VacationReasonsSortBy::Id, "id"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn defaults_are_sufficient() {
        VacationReasons::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = api::ignore(VacationReasons::builder().build().unwrap());

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/vacation-reasons")
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        endpoint.query(&client).unwrap();

        mock.assert();
    }

    #[test]
    fn endpoint_sort_ascending() {
        let endpoint = api::ignore(
            VacationReasons::builder()
                .sort(SortBy::Asc(VacationReasonsSortBy::Name))
                .build()
                .unwrap(),
        );

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/vacation-reasons")
            .query(vec![("sort".into(), "name".into())])
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        endpoint.query(&client).unwrap();

        mock.assert();
    }

    #[test]
    fn endpoint_sort_descending() {
        let endpoint = api::ignore(
            VacationReasons::builder()
                .sort(SortBy::Desc(VacationReasonsSortBy::Name))
                .build()
                .unwrap(),
        );

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/vacation-reasons")
            .query(vec![("sort".into(), "-name".into())])
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        endpoint.query(&client).unwrap();

        mock.assert();
    }

    #[test]
    fn endpoint_page() {
        let endpoint = api::ignore(VacationReasons::builder().page(2).build().unwrap());

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/vacation-reasons")
            .query(vec![("page".into(), "2".into())])
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        endpoint.query(&client).unwrap();

        mock.assert();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = api::ignore(VacationReasons::builder().name("test").build().unwrap());

        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/vacation-reasons")
            .query(vec![("name".into(), "test".into())])
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        endpoint.query(&client).unwrap();

        mock.assert();
    }

    #[test]
    fn endpoint_archived() {
        let endpoint = api::ignore(VacationReasons::builder().archived(true).build().unwrap());
        let expected = ExpectedRequest::builder()
            .method(Method::GET)
            .path("/vacation-reasons")
            .query(vec![("archived".into(), "1".into())])
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        endpoint.query(&client).unwrap();

        mock.assert();
    }
}
