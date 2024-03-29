use chrono::NaiveDate;
use derive_builder::Builder;
use http::Method;
use params::ParamValue;
use std::borrow::Cow;

use crate::api::{
    error::BodyError,
    header,
    params::{self, JsonParams},
    Endpoint,
};

#[derive(Debug, Builder, Clone)]
pub struct CreateTimeEntry<'a> {
    resource_id: u64,
    project_id: u64,
    minutes: u64,
    #[builder(default)]
    weekend_booking: Option<bool>,
    #[builder(default)]
    holidays_booking: Option<bool>,
    #[builder(default)]
    vacations_booking: Option<bool>,
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    #[builder(setter(into), default)]
    external_id: Option<Cow<'a, str>>,
    start_date: NaiveDate,
    end_date: NaiveDate,
    #[builder(default)]
    creator_resource_id: Option<u64>,
    #[builder(default)]
    editor_resource_id: Option<u64>,
    #[builder(default)]
    disable_notifications: Option<bool>,
}

impl<'a> CreateTimeEntry<'a> {
    pub fn builder() -> CreateTimeEntryBuilder<'a> {
        CreateTimeEntryBuilder::default()
    }
}

impl<'a> Endpoint for CreateTimeEntry<'a> {
    fn url(&self) -> Cow<'static, str> {
        "time-entries".into()
    }

    fn method(&self) -> http::Method {
        Method::POST
    }

    fn body(&self) -> Result<Option<Vec<u8>>, BodyError> {
        let mut params = JsonParams::default();

        params
            .push("resource_id", self.resource_id)?
            .push("project_id", self.project_id)?
            .push("minutes", self.minutes)?
            .push_opt("weekend_booking", self.weekend_booking)?
            .push_opt("holidays_booking", self.holidays_booking)?
            .push_opt("vacations_booking", self.vacations_booking)?
            .push_param_value_opt("description", self.description.as_ref())?
            .push_param_value_opt("external_id", self.external_id.as_ref())?
            .push("start_date", self.start_date.as_value())?
            .push("end_date", self.end_date.as_value())?
            .push_opt("creator_resource_id", self.creator_resource_id)?
            .push_opt("editor_resource_id", self.editor_resource_id)?;

        Ok(Some(params.to_body()?))
    }

    fn headers(&self) -> Option<http::HeaderMap> {
        let mut headers = http::HeaderMap::new();
        if let Some(disable_notifications) = self.disable_notifications {
            header::disable_notifications_header(&mut headers, disable_notifications);
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
    use chrono::NaiveDate;
    use http::Method;
    use serde_json::json;

    #[test]
    fn create_time_entry_request() {
        let endpoint = api::ignore(
            CreateTimeEntry::builder()
                .resource_id(1)
                .project_id(2)
                .minutes(3)
                .weekend_booking(Some(true))
                .holidays_booking(Some(true))
                .vacations_booking(Some(true))
                .description(Some("description".into()))
                .external_id(Some("external_id".into()))
                .start_date(NaiveDate::from_ymd(2020, 1, 1))
                .end_date(NaiveDate::from_ymd(2020, 1, 2))
                .creator_resource_id(Some(4))
                .editor_resource_id(Some(5))
                .build()
                .unwrap(),
        );

        let expected = ExpectedRequest::builder()
            .method(Method::POST)
            .path("/time-entries")
            .request_body(json!({
                "resource_id": 1,
                "project_id": 2,
                "minutes": 3,
                "weekend_booking": true,
                "holidays_booking": true,
                "vacations_booking": true,
                "description": "description",
                "external_id": "external_id",
                "start_date": "2020-01-01",
                "end_date": "2020-01-02",
                "creator_resource_id": 4,
                "editor_resource_id": 5,
            }))
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn create_time_entry_minimal_request() {
        let endpoint = api::ignore(
            CreateTimeEntry::builder()
                .resource_id(1)
                .project_id(2)
                .minutes(3)
                .start_date(NaiveDate::from_ymd(2020, 1, 1))
                .end_date(NaiveDate::from_ymd(2020, 1, 2))
                .build()
                .unwrap(),
        );

        let expected = ExpectedRequest::builder()
            .method(Method::POST)
            .path("/time-entries")
            .request_body(json!({
                "resource_id": 1,
                "project_id": 2,
                "minutes": 3,
                "start_date": "2020-01-01",
                "end_date": "2020-01-02",
            }))
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }

    #[test]
    fn missing_resource_id() {
        let endpoint = CreateTimeEntry::builder()
            .project_id(2)
            .minutes(3)
            .start_date(NaiveDate::from_ymd(2020, 1, 1))
            .end_date(NaiveDate::from_ymd(2020, 1, 2))
            .build();

        assert!(endpoint.is_err());
    }

    #[test]
    fn missing_project_id() {
        let endpoint = CreateTimeEntry::builder()
            .resource_id(1)
            .minutes(3)
            .start_date(NaiveDate::from_ymd(2020, 1, 1))
            .end_date(NaiveDate::from_ymd(2020, 1, 2))
            .build();

        assert!(endpoint.is_err());
    }

    #[test]
    fn missing_minutes() {
        let endpoint = CreateTimeEntry::builder()
            .resource_id(1)
            .project_id(2)
            .start_date(NaiveDate::from_ymd(2020, 1, 1))
            .end_date(NaiveDate::from_ymd(2020, 1, 2))
            .build();

        assert!(endpoint.is_err());
    }

    #[test]
    fn missing_start_date() {
        let endpoint = CreateTimeEntry::builder()
            .resource_id(1)
            .project_id(2)
            .minutes(3)
            .end_date(NaiveDate::from_ymd(2020, 1, 2))
            .build();

        assert!(endpoint.is_err());
    }

    #[test]
    fn missing_end_date() {
        let endpoint = CreateTimeEntry::builder()
            .resource_id(1)
            .project_id(2)
            .minutes(3)
            .start_date(NaiveDate::from_ymd(2020, 1, 1))
            .build();

        assert!(endpoint.is_err());
    }

    #[test]
    fn disable_notifications() {
        let endpoint = api::ignore(
            CreateTimeEntry::builder()
                .resource_id(1)
                .project_id(2)
                .minutes(3)
                .start_date(NaiveDate::from_ymd(2020, 1, 1))
                .end_date(NaiveDate::from_ymd(2020, 1, 2))
                .disable_notifications(Some(true))
                .build()
                .unwrap(),
        );

        let expected = ExpectedRequest::builder()
            .method(Method::POST)
            .path("/time-entries")
            .request_headers(vec![(DISABLE_NOTIFICATION_HEADER.into(), "true".into())])
            .build()
            .unwrap();

        let client = TestClient::expecting(expected);

        endpoint.query(&client).unwrap();
    }
}
