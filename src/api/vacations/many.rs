use crate::api::{paged::Pageable, params::ParamValue, sort_by::SortBy, Endpoint, QueryParams};
use chrono::NaiveDate;
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VacationsSortBy {
    ResourceId,
    Status,
    PeriodId,
    RequestedApproverId,
    ReasonId,
    Description,
    ExternalId,
    StartDate,
    EndDate,
    CreatorResourceId,
    ApproverResourceId,
    EditorResourceId,
}

impl Default for VacationsSortBy {
    fn default() -> Self {
        Self::StartDate
    }
}

impl VacationsSortBy {
    fn as_str(self) -> &'static str {
        match self {
            Self::ResourceId => "resource_id",
            Self::Status => "status",
            Self::PeriodId => "period_id",
            Self::RequestedApproverId => "requested_approver_id",
            Self::ReasonId => "reason_id",
            Self::Description => "description",
            Self::ExternalId => "external_id",
            Self::StartDate => "start_date",
            Self::EndDate => "end_date",
            Self::CreatorResourceId => "creator_resource_id",
            Self::ApproverResourceId => "approver_resource_id",
            Self::EditorResourceId => "editor_resource_id",
        }
    }
}

impl ParamValue<'static> for VacationsSortBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Vacations {
    #[builder(default)]
    sort: Option<SortBy<VacationsSortBy>>,
    #[builder(default)]
    page: Option<u64>,
    #[builder(default)]
    resource_id: Option<Vec<u64>>,
    #[builder(default)]
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

impl Vacations {
    pub fn builder() -> VacationsBuilder {
        VacationsBuilder::default()
    }
}

impl Endpoint for Vacations {
    fn url(&self) -> Cow<'static, str> {
        "vacations".into()
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("sort", self.sort)
            .push_opt("page", self.page)
            .push_opt("resource_id", self.resource_id.clone())
            .push_opt("external_id", self.external_id.clone())
            .push_opt("start_date_from", self.start_date_from)
            .push_opt("start_date_to", self.start_date_to)
            .push_opt("end_date_from", self.end_date_from)
            .push_opt("end_date_to", self.end_date_to)
            .push_opt("date", self.date);

        params
    }
}

impl Pageable for Vacations {}
