pub mod api;
pub mod teamdeck;

#[cfg(test)]
mod test;

pub use crate::teamdeck::AsyncTeamdeck;
pub use crate::teamdeck::Teamdeck;
