use crate::api::{
    paged::Pageable,
    params::{self, ParamValue},
    sort_by::SortBy,
    Endpoint, QueryParams,
};
use chrono::NaiveDate;
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolidaysSortBy {
    Name,
    Date,
    OrganizationUnitId,
}

impl Default for HolidaysSortBy {
    fn default() -> Self {
        Self::Date
    }
}

impl HolidaysSortBy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Date => "date",
            Self::OrganizationUnitId => "organization_unit_id",
        }
    }
}

impl ParamValue<'static> for HolidaysSortBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Holidays<'a> {
    #[builder(default)]
    sort: Option<SortBy<HolidaysSortBy>>,
    #[builder(default)]
    page: Option<u64>,
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    #[builder(default)]
    from: Option<NaiveDate>,
    #[builder(default)]
    to: Option<NaiveDate>,
}

impl<'a> Holidays<'a> {
    pub fn builder() -> HolidaysBuilder<'a> {
        HolidaysBuilder::default()
    }
}

impl<'a> Endpoint for Holidays<'a> {
    fn url(&self) -> Cow<'static, str> {
        "holidays".into()
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("sort", self.sort)
            .push_opt("page", self.page)
            .push_opt("name", self.name.as_ref())
            .push_opt("from", self.from)
            .push_opt("to", self.to);

        params
    }
}

impl<'a> Pageable for Holidays<'a> {}
