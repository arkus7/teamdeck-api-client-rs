use derive_builder::Builder;
use std::borrow::Cow;

use crate::api::{Endpoint, QueryParams};

use super::projects::ProjectsExpand;

#[derive(Debug, Builder)]
pub struct Project {
    id: usize,
    #[builder(default)]
    expand: Option<ProjectsExpand>,
}

impl Project {
    pub fn builder() -> ProjectBuilder {
        ProjectBuilder::default()
    }
}

impl Endpoint for Project {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn url(&self) -> Cow<'static, str> {
        format!("projects/{}", self.id).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("expand", self.expand);

        params
    }
}

// TODO: Add tests
