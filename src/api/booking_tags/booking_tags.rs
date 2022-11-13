use crate::api::{paged::Pageable, params::ParamValue, Endpoint, QueryParams, sort_by::SortBy};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookingTagsSortBy {
    Id,
    Name,
    Icon,
    Color,
}

impl Default for BookingTagsSortBy {
    fn default() -> Self {
        Self::Id
    }
}

impl BookingTagsSortBy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Name => "name",
            Self::Icon => "icon",
            Self::Color => "color",
        }
    }
}

impl ParamValue<'static> for BookingTagsSortBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct BookingTags<'a> {
    #[builder(default)]
    sort: Option<SortBy<BookingTagsSortBy>>,
    #[builder(default)]
    page: Option<u64>,
    #[builder(default)]
    archived: Option<bool>,
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,
}

impl<'a> BookingTags<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> BookingTagsBuilder<'a> {
        BookingTagsBuilder::default()
    }
}

impl<'a> Endpoint for BookingTags<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn url(&self) -> Cow<'static, str> {
        "booking-tags".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("sort", self.sort);
        params.push_opt("page", self.page);
        params.push_opt("archived", self.archived);
        params.push_opt("name", self.name.as_ref());
        params
    }
}

impl Pageable for BookingTags<'_> {}

// TODO: Add tests
