use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;

#[derive(Debug, Builder)]
pub struct VacationPeriod {
    id: u64,
}

impl VacationPeriod {
    pub fn builder() -> VacationPeriodBuilder {
        VacationPeriodBuilder::default()
    }
}

impl Endpoint for VacationPeriod {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn url(&self) -> Cow<'static, str> {
        format!("vacation-periods/{}", self.id).into()
    }
}

// TODO: Add tests
