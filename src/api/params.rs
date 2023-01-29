use std::{borrow::Cow, collections::HashMap};

use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use serde_json::Value;
use url::Url;

pub trait ParamValue<'a> {
    fn as_value(&self) -> Cow<'a, str>;
}

impl ParamValue<'static> for bool {
    fn as_value(&self) -> Cow<'static, str> {
        if *self {
            "1".into()
        } else {
            "0".into()
        }
    }
}

impl<'a> ParamValue<'a> for &'a str {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl ParamValue<'static> for String {
    fn as_value(&self) -> Cow<'static, str> {
        self.clone().into()
    }
}

impl<'a> ParamValue<'a> for &'a String {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl<'a> ParamValue<'a> for Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        self.clone()
    }
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).clone()
    }
}

impl ParamValue<'static> for u64 {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self).into()
    }
}

impl ParamValue<'static> for f64 {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self).into()
    }
}

impl ParamValue<'static> for DateTime<Utc> {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            .into()
    }
}

impl ParamValue<'static> for NaiveDate {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self.format("%Y-%m-%d")).into()
    }
}

impl<T> ParamValue<'static> for Vec<T>
where
    T: ParamValue<'static>,
{
    fn as_value(&self) -> Cow<'static, str> {
        self.iter()
            .map(|v| v.as_value())
            .collect::<Vec<_>>()
            .join(",")
            .into()
    }
}

#[derive(Debug, Default, Clone)]
pub struct QueryParams<'a> {
    params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> QueryParams<'a> {
    /// Push a single parameter.
    pub fn push<'b, K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params.push((key.into(), value.as_value()));
        self
    }

    /// Push a single parameter.
    pub fn push_opt<'b, K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        if let Some(value) = value {
            self.params.push((key.into(), value.as_value()));
        }
        self
    }

    /// Push a set of parameters.
    pub fn extend<'b, I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params
            .extend(iter.map(|(key, value)| (key.into(), value.as_value())));
        self
    }

    /// Add the parameters to a URL.
    pub fn add_to_url(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();
        pairs.extend_pairs(self.params.iter());
    }
}

#[derive(Debug, Default, Clone)]
pub struct JsonParams<'a> {
    params: HashMap<Cow<'a, str>, Value>,
}

impl<'a> JsonParams<'a> {
    pub fn push<'b, K, V>(&mut self, key: K, value: V) -> Result<&mut Self, serde_json::Error>
    where
        K: Into<Cow<'a, str>>,
        V: Serialize,
        'b: 'a,
    {
        self.params.insert(key.into(), serde_json::to_value(value)?);
        Ok(self)
    }

    pub fn push_opt<'b, K, V>(
        &mut self,
        key: K,
        value: Option<V>,
    ) -> Result<&mut Self, serde_json::Error>
    where
        K: Into<Cow<'a, str>>,
        V: Serialize,
        'b: 'a,
    {
        if let Some(value) = value {
            self.params.insert(key.into(), serde_json::to_value(value)?);
        }
        Ok(self)
    }

    pub fn push_param_value<'b, K, V>(
        &mut self,
        key: K,
        value: V,
    ) -> Result<&mut Self, serde_json::Error>
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params
            .insert(key.into(), serde_json::to_value(value.as_value())?);
        Ok(self)
    }

    pub fn push_param_value_opt<'b, K, V>(
        &mut self,
        key: K,
        value: Option<V>,
    ) -> Result<&mut Self, serde_json::Error>
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        if let Some(value) = value {
            self.params
                .insert(key.into(), serde_json::to_value(value.as_value())?);
        }
        Ok(self)
    }

    pub fn to_body(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self.params)
    }
}
