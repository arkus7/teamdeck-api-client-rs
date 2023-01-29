use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::{header::disable_notifications_header, Endpoint};

#[derive(Debug, Builder)]
pub struct DeleteTimeEntry {
    id: usize,
    #[builder(default)]
    disable_notifications: Option<bool>,
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

    fn headers(&self) -> Option<http::HeaderMap> {
        let mut headers = http::HeaderMap::new();
        if let Some(disable_notifications) = self.disable_notifications {
            disable_notifications_header(&mut headers, disable_notifications);
        }
        Some(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, header::DISABLE_NOTIFICATION_HEADER, Query},
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

    #[test]
    fn disable_notifications() {
        let endpoint = api::ignore(
            DeleteTimeEntry::builder()
                .id(1)
                .disable_notifications(Some(true))
                .build()
                .unwrap(),
        );

        let expected = ExpectedRequest::builder()
            .method(Method::DELETE)
            .path("/time-entries/1")
            .request_headers(vec![(DISABLE_NOTIFICATION_HEADER.into(), "true".into())])
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }
}
