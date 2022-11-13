use std::borrow::Cow;

use super::ParamValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy<T>
where
    T: ParamValue<'static>,
{
    Ascending(T),
    Descending(T),
}

impl<T> Default for SortBy<T>
where
    T: ParamValue<'static>,
    T: Default,
{
    fn default() -> Self {
        Self::Ascending(T::default())
    }
}

impl<T> ParamValue<'static> for SortBy<T>
where
    T: ParamValue<'static>,
{
    fn as_value(&self) -> Cow<'static, str> {
        match self {
            SortBy::Ascending(val) => val.as_value(),
            SortBy::Descending(val) => format!("-{}", val.as_value()).into(),
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
        assert_eq!(sort_by, SortBy::<DummySortBy>::Ascending(DummySortBy::Name))
    }

    #[test]
    fn returns_inner_param_value_when_ascending() {
        let sort_by_values = vec![
            SortBy::Ascending(DummySortBy::Name),
            SortBy::Ascending(DummySortBy::Email),
            SortBy::Ascending(DummySortBy::Active),
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
            SortBy::Descending(DummySortBy::Name),
            SortBy::Descending(DummySortBy::Email),
            SortBy::Descending(DummySortBy::Active),
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
