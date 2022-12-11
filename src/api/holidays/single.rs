use derive_builder::Builder;
use http::Method;

use crate::api::Endpoint;

#[derive(Debug, Builder)]
pub struct Holiday {
    id: usize,
}

impl Holiday {
    pub fn builder() -> HolidayBuilder {
        HolidayBuilder::default()
    }
}

impl Endpoint for Holiday {
    fn url(&self) -> std::borrow::Cow<'static, str> {
        format!("holidays/{}", self.id).into()
    }

    fn method(&self) -> http::Method {
        Method::GET
    }
}

// TODO: Add tests
