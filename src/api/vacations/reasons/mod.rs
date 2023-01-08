mod many;
mod single;

pub use many::{
    VacationReasons, VacationReasonsBuilder, VacationReasonsBuilderError, VacationReasonsSortBy,
};
pub use single::{VacationReason, VacationReasonBuilder, VacationReasonBuilderError};
