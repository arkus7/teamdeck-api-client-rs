use super::{
    client::{AsyncClient, Client},
    error::ApiError,
};
use async_trait::async_trait;

pub trait Query<T, C>
where
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

/// A trait which represents an asynchronous query which may be made to a Teamdeck client.
#[async_trait]
pub trait AsyncQuery<T, C>
where
    C: AsyncClient,
{
    /// Perform the query asynchronously against the client.
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
