use std::{borrow::Cow};

use super::ParamValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy<T>
where
    T: ParamValue<'static>,
{
    Asc(T),
    Desc(T),
}

impl<T> Default for SortBy<T>
where
    T: ParamValue<'static>,
    T: Default,
{
    fn default() -> Self {
        Self::Asc(T::default())
    }
}

impl<T> ParamValue<'static> for SortBy<T>
where
    T: ParamValue<'static>,
{
    fn as_value(&self) -> Cow<'static, str> {
        match self {
            SortBy::Asc(val) => val.as_value(),
            SortBy::Desc(val) => format!("-{}", val.as_value()).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum DummySortBy {
        Name,
        Email,
        Active,
    }

    impl Default for DummySortBy {
        fn default() -> Self {
            Self::Name
        }
    }

    impl ParamValue<'static> for DummySortBy {
        fn as_value(&self) -> Cow<'static, str> {
            match self {
                Self::Name => "name".into(),
                Self::Email => "email".into(),
                Self::Active => "active".into(),
            }
        }
    }

    #[test]
    fn ascending_inner_default_by_default() {
        let sort_by: SortBy<DummySortBy> = Default::default();
        assert_eq!(sort_by, SortBy::<DummySortBy>::Asc(DummySortBy::Name))
    }

    #[test]
    fn returns_inner_param_value_when_ascending() {
        let sort_by_values = vec![
            SortBy::Asc(DummySortBy::Name),
            SortBy::Asc(DummySortBy::Email),
            SortBy::Asc(DummySortBy::Active),
        ];

        let expected_values: Vec<Cow<str>> = vec!["name".into(), "email".into(), "active".into()];
        let param_values = sort_by_values
            .iter()
            .map(SortBy::as_value)
            .collect::<Vec<_>>();

        assert_eq!(param_values, expected_values)
    }

    #[test]
    fn adds_hyphen_prefix_to_inner_values_when_descending() {
        let sort_by_values = vec![
            SortBy::Desc(DummySortBy::Name),
            SortBy::Desc(DummySortBy::Email),
            SortBy::Desc(DummySortBy::Active),
        ];

        let expected_values: Vec<Cow<str>> =
            vec!["-name".into(), "-email".into(), "-active".into()];
        let param_values = sort_by_values
            .iter()
            .map(SortBy::as_value)
            .collect::<Vec<_>>();

        assert_eq!(param_values, expected_values)
    }
}
