mod many;
mod single;

pub use many::{
    VacationPeriods, VacationPeriodsBuilder, VacationPeriodsBuilderError, VacationPeriodsSortBy,
};
pub use single::{VacationPeriod, VacationPeriodBuilder, VacationPeriodBuilderError};
