use http::{HeaderMap, Request};
use serde::de::DeserializeOwned;

use super::{endpoint::url_to_http_uri, ApiError, Client, Endpoint, Query};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pagination {
    All,
}

impl Default for Pagination {
    fn default() -> Self {
        Self::All
    }
}

const MAX_PAGE_SIZE: u8 = 10;

impl Pagination {
    pub(crate) fn page_limit(self) -> u8 {
        match self {
            Pagination::All => MAX_PAGE_SIZE,
        }
    }

    pub(crate) fn is_last_page(self, last_page_size: u8) -> bool {
        // If the last page didn't return any results, we're done.
        if last_page_size == 0 {
            return true;
        }

        // If the last page has fewer elements than our limit, we're definitely done.
        if last_page_size < self.page_limit() {
            return true;
        }

        // We're not done yet.
        false
    }
}

pub struct Paged<E> {
    endpoint: E,
    pagination: Pagination,
}

pub fn paged<E>(endpoint: E, pagination: Pagination) -> Paged<E> {
    Paged {
        endpoint,
        pagination,
    }
}

pub trait Pageable {}

impl<E, T, C> Query<Vec<T>, C> for Paged<E>
where
    E: Endpoint,
    E: Pageable,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, super::ApiError<<C>::Error>> {
        let mut page_num = 1;
        let mut results = vec![];

        let url = {
          let mut url = client.rest_endpoint(&self.endpoint.url())?;
          self.endpoint.parameters().add_to_url(&mut url);
          url
      };

        loop {
            let page_url = {
                let page_str = format!("{}", page_num);
                let mut page_url = url.clone();

                {
                    let mut pairs = page_url.query_pairs_mut();
                    pairs.append_pair("page", &page_str);
                }

                page_url
            };

            dbg!(&page_url);

            let request = Request::builder()
                .method(self.endpoint.method())
                .uri(url_to_http_uri(page_url))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json");

            let body = self.endpoint.body().unwrap_or_default();

            let response = client.rest(request, body)?;
            let status = response.status();
            let value = if let Ok(val) = serde_json::from_slice(&response.body()) {
                val
            } else {
                return Err(ApiError::server_error(status, response.body()));
            };

            if !status.is_success() {
                return Err(ApiError::from_teamdeck(value));
            }

            let page =
                serde_json::from_value::<Vec<T>>(value).map_err(ApiError::data_type::<Vec<T>>)?;
            let page_len = page.len() as u8;

            let is_last_page = {
                results.extend(page);
                self.pagination.is_last_page(page_len)
            };

            if is_last_page {
                break;
            }

            page_num += 1;
        }

        Ok(results)
    }
}
