use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;

#[derive(Debug, Builder)]
pub struct Vacation {
    id: u64,
}

impl Vacation {
    pub fn builder() -> VacationBuilder {
        VacationBuilder::default()
    }
}

impl Endpoint for Vacation {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn url(&self) -> Cow<'static, str> {
        format!("vacations/{}", self.id).into()
    }
}

// TODO: Add tests
