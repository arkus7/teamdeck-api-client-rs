use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;

#[derive(Debug, Builder)]
pub struct TimeEntryTag {
    id: usize,
}

impl TimeEntryTag {
    pub fn builder() -> TimeEntryTagBuilder {
        TimeEntryTagBuilder::default()
    }
}

impl Endpoint for TimeEntryTag {
    fn url(&self) -> Cow<'static, str> {
        format!("time-entry-tags/{}", self.id).into()
    }

    fn method(&self) -> http::Method {
        Method::GET
    }
}

// TODO: Add tests
