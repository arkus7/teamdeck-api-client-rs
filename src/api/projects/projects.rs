use crate::api::{paged::Pageable, params::ParamValue, sort_by::SortBy, Endpoint, QueryParams};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectsSortBy {
    Name,
    Color,
    Archived,
}

impl Default for ProjectsSortBy {
    fn default() -> Self {
        Self::Name
    }
}

impl ProjectsSortBy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Color => "color",
            Self::Archived => "archived",
        }
    }
}

impl ParamValue<'static> for ProjectsSortBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProjectsExpand {
    Tags,
}

impl ProjectsExpand {
    fn as_str(self) -> &'static str {
        match self {
            Self::Tags => "tags",
        }
    }
}

impl ParamValue<'static> for ProjectsExpand {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Projects<'a> {
    #[builder(default)]
    sort: Option<SortBy<ProjectsSortBy>>,
    #[builder(default)]
    page: Option<u64>,
    #[builder(default)]
    archived: Option<bool>,
    #[builder(default)]
    expand: Option<ProjectsExpand>,
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,
}

impl<'a> Projects<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectsBuilder<'a> {
        ProjectsBuilder::default()
    }
}

impl<'a> Endpoint for Projects<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn url(&self) -> Cow<'static, str> {
        "projects".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("sort", self.sort);
        params.push_opt("page", self.page);
        params.push_opt("archived", self.archived);
        params.push_opt("expand", self.expand);
        params.push_opt("name", self.name.as_ref());
        params
    }
}

// TODO: Add tests
