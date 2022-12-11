use derive_builder::Builder;
use http::Method;
use params::QueryParams;
use std::borrow::Cow;

use crate::api::{params, Endpoint};

use super::many::TimeEntriesExpand;

#[derive(Debug, Builder)]
pub struct TimeEntry {
    id: usize,
    #[builder(default)]
    expand: Option<TimeEntriesExpand>,
}

impl TimeEntry {
    pub fn builder() -> TimeEntryBuilder {
        TimeEntryBuilder::default()
    }
}

impl Endpoint for TimeEntry {
    fn url(&self) -> Cow<'static, str> {
        format!("time-entries/{}", self.id).into()
    }

    fn method(&self) -> http::Method {
        Method::GET
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("expand", self.expand);

        params
    }
}

// TODO: Add tests
