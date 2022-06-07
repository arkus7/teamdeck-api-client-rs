pub enum Pagination {
  All,
  Limit(u64),
}

pub struct Paged<E> {
  endpoint: E,
  pagination: Pagination
}
