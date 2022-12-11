use crate::api::{paged::Pageable, params::ParamValue, sort_by::SortBy, Endpoint, QueryParams};
use chrono::NaiveDate;
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeEntriesSortBy {
    ResourceId,
    ProjectId,
    Minutes,
    WeekendBooking,
    HolidaysBooking,
    VacationsBooking,
    Description,
    ExternalId,
    StartDate,
    EndDate,
    CreatorResourceId,
    EditorResourceId,
}

impl Default for TimeEntriesSortBy {
    fn default() -> Self {
        Self::StartDate
    }
}

impl TimeEntriesSortBy {
    fn as_str(self) -> &'static str {
        match self {
            TimeEntriesSortBy::ResourceId => "resource_id",
            TimeEntriesSortBy::ProjectId => "project_id",
            TimeEntriesSortBy::Minutes => "minutes",
            TimeEntriesSortBy::WeekendBooking => "weekend_booking",
            TimeEntriesSortBy::HolidaysBooking => "holidays_booking",
            TimeEntriesSortBy::VacationsBooking => "vacations_booking",
            TimeEntriesSortBy::Description => "description",
            TimeEntriesSortBy::ExternalId => "external_id",
            TimeEntriesSortBy::StartDate => "start_date",
            TimeEntriesSortBy::EndDate => "end_date",
            TimeEntriesSortBy::CreatorResourceId => "creator_resource_id",
            TimeEntriesSortBy::EditorResourceId => "editor_resource_id",
        }
    }
}

impl ParamValue<'static> for TimeEntriesSortBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TimeEntriesExpand {
    Tags,
}

impl TimeEntriesExpand {
    fn as_str(self) -> &'static str {
        match self {
            TimeEntriesExpand::Tags => "tags",
        }
    }
}

impl ParamValue<'static> for TimeEntriesExpand {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct TimeEntries {
    #[builder(default)]
    sort: Option<SortBy<TimeEntriesSortBy>>,
    #[builder(default)]
    page: Option<u64>,
    #[builder(default)]
    expand: Option<TimeEntriesExpand>,
    #[builder(default)]
    resource_id: Option<Vec<u64>>,
    #[builder(default)]
    project_id: Option<Vec<u64>>,
    #[builder(setter(into), default)]
    external_id: Option<Vec<String>>,
    #[builder(default)]
    start_date_from: Option<NaiveDate>,
    #[builder(default)]
    start_date_to: Option<NaiveDate>,
    #[builder(default)]
    end_date_from: Option<NaiveDate>,
    #[builder(default)]
    end_date_to: Option<NaiveDate>,
    #[builder(default)]
    date: Option<NaiveDate>,
}

impl TimeEntries {
    pub fn builder() -> TimeEntriesBuilder {
        TimeEntriesBuilder::default()
    }
}

impl Endpoint for TimeEntries {
    fn url(&self) -> Cow<'static, str> {
        "time-entries".into()
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("sort", self.sort)
            .push_opt("page", self.page)
            .push_opt("expand", self.expand)
            .push_opt("resource_id", self.resource_id.clone())
            .push_opt("project_id", self.project_id.clone())
            .push_opt("external_id", self.external_id.clone())
            .push_opt("start_date_from", self.start_date_from)
            .push_opt("start_date_to", self.start_date_to)
            .push_opt("end_date_from", self.end_date_from)
            .push_opt("end_date_to", self.end_date_to)
            .push_opt("date", self.date);

        params
    }
}

impl Pageable for TimeEntries {}

// TODO: Add tests
