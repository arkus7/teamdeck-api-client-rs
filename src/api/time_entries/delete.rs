use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::{params, Endpoint};

#[derive(Debug, Builder)]
pub struct DeleteTimeEntry {
    id: usize,
}

impl DeleteTimeEntry {
    pub fn builder() -> DeleteTimeEntryBuilder {
        DeleteTimeEntryBuilder::default()
    }
}

impl Endpoint for DeleteTimeEntry {
    fn url(&self) -> Cow<'static, str> {
        format!("time-entries/{}", self.id).into()
    }

    fn method(&self) -> http::Method {
        Method::DELETE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query},
        test::client::{ExpectedRequest, TestClient},
    };

    #[test]
    fn delete_time_entry() {
        let endpoint = api::ignore(DeleteTimeEntry::builder().id(1).build().unwrap());

        let expected = ExpectedRequest::builder()
            .method(Method::DELETE)
            .path("/time-entries/1")
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn missing_id() {
        let endpoint = DeleteTimeEntry::builder().build();

        assert!(endpoint.is_err());
    }
}
