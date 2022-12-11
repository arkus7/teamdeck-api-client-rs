use crate::api::{paged::Pageable, params::ParamValue, sort_by::SortBy, Endpoint, QueryParams};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VacationPeriodsSortBy {
    Id,
    Name,
    Archived,
    HoursPerDay,
}

impl Default for VacationPeriodsSortBy {
    fn default() -> Self {
        Self::Name
    }
}

impl VacationPeriodsSortBy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Name => "name",
            Self::Archived => "archived",
            Self::HoursPerDay => "hours_per_day",
        }
    }
}

impl ParamValue<'static> for VacationPeriodsSortBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct VacationPeriods<'a> {
    #[builder(default)]
    sort: Option<SortBy<VacationPeriodsSortBy>>,
    #[builder(default)]
    page: Option<u64>,
    #[builder(default)]
    name: Option<Cow<'a, str>>,
    #[builder(default)]
    archived: Option<bool>,
}

impl<'a> VacationPeriods<'a> {
    pub fn builder() -> VacationPeriodsBuilder<'a> {
        VacationPeriodsBuilder::default()
    }
}

impl<'a> Endpoint for VacationPeriods<'a> {
    fn url(&self) -> Cow<'static, str> {
        "vacation-periods".into()
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
            .push_opt("archived", self.archived);

        params
    }
}

impl<'a> Pageable for VacationPeriods<'a> {}
