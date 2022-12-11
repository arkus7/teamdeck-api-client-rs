use std::borrow::Cow;

use derive_builder::Builder;
use http::Method;

use crate::api::{Endpoint, QueryParams};

use super::many::BookingsExpand;

#[derive(Debug, Builder)]
pub struct Booking {
    id: usize,
    #[builder(default)]
    expand: Option<BookingsExpand>,
}

impl Booking {
    pub fn builder() -> BookingBuilder {
        BookingBuilder::default()
    }
}

impl Endpoint for Booking {
    fn url(&self) -> Cow<'static, str> {
        format!("bookings/{}", self.id).into()
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
