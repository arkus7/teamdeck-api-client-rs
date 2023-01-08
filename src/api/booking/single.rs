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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedRequest, TestClient};

    #[test]
    fn booking_id_is_required() {
        assert!(Booking::builder().build().is_err());
    }

    #[test]
    fn booking() {
        let endpoint = api::ignore(Booking::builder().id(1).build().unwrap());

        let expected = ExpectedRequest::builder()
            .path("/bookings/1")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn booking_expand() {
        let endpoint = api::ignore(
            Booking::builder()
                .id(1)
                .expand(Some(BookingsExpand::Tags))
                .build()
                .unwrap(),
        );

        let expected = ExpectedRequest::builder()
            .path("/bookings/1")
            .query("expand=tags")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }
}
