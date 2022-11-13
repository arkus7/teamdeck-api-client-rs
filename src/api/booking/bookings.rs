use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::{paged::Pageable, sort::SortDirection, Endpoint, ParamValue, QueryParams};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum BookingsSortBy {
    StartDate(SortDirection),
    EndDate(SortDirection),
    Minutes,
    ResourceId,
    ProjectId,
}

impl Default for BookingsSortBy {
    fn default() -> Self {
        Self::StartDate(SortDirection::Descending)
    }
}

impl BookingsSortBy {
    fn as_str(self) -> &'static str {
        match self {
            Self::StartDate(direction) => match direction {
                SortDirection::Ascending => "start_date",
                SortDirection::Descending => "-start_date",
            },
            Self::EndDate(direction) => match direction {
                SortDirection::Ascending => "end_date",
                SortDirection::Descending => "-end_date",
            },
            Self::Minutes => "minutes",
            Self::ResourceId => "resource_id",
            Self::ProjectId => "project_id",
        }
    }
}

impl ParamValue<'static> for BookingsSortBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BookingsExpand {
    Tags,
}

impl BookingsExpand {
    fn as_str(self) -> &'static str {
        match self {
            Self::Tags => "tags",
        }
    }
}

impl ParamValue<'static> for BookingsExpand {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Bookings<'a> {
    #[builder(default)]
    sort: Option<BookingsSortBy>,
    #[builder(default)]
    expand: Option<BookingsExpand>,
    #[builder(default)]
    page: Option<u64>,
    #[builder(default)]
    resource_id: Option<u64>,
    #[builder(default)]
    project_id: Option<u64>,
    #[builder(default)]
    external_id: Option<Cow<'a, str>>,
    #[builder(default)]
    start_date_from: Option<chrono::NaiveDate>,
    #[builder(default)]
    start_date_to: Option<chrono::NaiveDate>,
    #[builder(default)]
    end_date_from: Option<chrono::NaiveDate>,
    #[builder(default)]
    end_date_to: Option<chrono::NaiveDate>,
    #[builder(default)]
    date: Option<chrono::NaiveDate>,
}

impl<'a> Bookings<'a> {
    pub fn builder() -> BookingsBuilder<'a> {
        BookingsBuilder::default()
    }
}

impl<'a> Endpoint for Bookings<'a> {
    fn url(&self) -> Cow<'static, str> {
        "bookings".into()
    }

    fn method(&self) -> http::Method {
        Method::GET
    }

    fn parameters(&self) -> crate::api::QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("sort", self.sort)
            .push_opt("expand", self.expand)
            .push_opt("page", self.page)
            .push_opt("resource_id", self.resource_id)
            .push_opt("project_id", self.project_id)
            .push_opt("external_id", self.external_id.as_ref())
            .push_opt("start_date_from", self.start_date_from)
            .push_opt("start_date_to", self.start_date_to)
            .push_opt("end_date_from", self.end_date_from)
            .push_opt("end_date_to", self.end_date_to)
            .push_opt("date", self.date);

        params
    }
}

impl<'a> Pageable for Bookings<'a> {}

// TODO: add tests
