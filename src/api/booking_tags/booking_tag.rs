use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;

#[derive(Debug, Builder)]
pub struct BookingTag {
    id: usize,
}

impl BookingTag {
    pub fn builder() -> BookingTagBuilder {
        BookingTagBuilder::default()
    }
}

impl Endpoint for BookingTag {
    fn url(&self) -> Cow<'static, str> {
        format!("booking-tags/{}", self.id).into()
    }

    fn method(&self) -> http::Method {
        Method::GET
    }
}

// TODO: Add tests
