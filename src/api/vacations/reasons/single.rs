use derive_builder::Builder;
use std::borrow::Cow;

use crate::api::Endpoint;

#[derive(Debug, Builder)]
pub struct VacationReason {
    id: usize,
}

impl VacationReason {
    pub fn builder() -> VacationReasonBuilder {
        VacationReasonBuilder::default()
    }
}

impl Endpoint for VacationReason {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn url(&self) -> Cow<'static, str> {
        format!("vacation-reasons/{}", self.id).into()
    }
}

#[cfg(test)]
mod tests {
    use super::VacationReason;
    use crate::{
        api::{self, Query},
        test::client::{ExpectedRequest, TestClient},
    };

    #[test]
    fn vacation_id() {
        let endpoint = api::ignore(VacationReason::builder().id(1).build().unwrap());

        let expected = ExpectedRequest::builder()
            .path("/vacation-reasons/1")
            .build()
            .unwrap();

        let client = TestClient::new();
        let mock = client.expect(expected);

        endpoint.query(&client).unwrap();

        mock.assert();
    }
}
