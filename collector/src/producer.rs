use core::fmt;
use std::ops::Deref;

use anyhow::{anyhow, Context, Error};
use axum::{
    headers,
    http::header::{HeaderName, HeaderValue},
};
use lazy_static::lazy_static;

pub const HEADER_NAME: &str = "mekatek-producer";
lazy_static! {
    pub static ref HEADER: HeaderName = HeaderName::from_static(HEADER_NAME);
}

#[derive(Clone, Debug, PartialEq)]
pub struct Producer(String);

impl Deref for Producer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Producer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl headers::Header for Producer {
    fn name() -> &'static HeaderName {
        &HEADER
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .next()
            .context("missing header value")
            .and_then(|val| {
                val.to_str()
                    .context("failed to convert header value to str")
            })
            .and_then(Self::try_from)
            .map_err(|_| headers::Error::invalid())
    }

    fn encode<E: Extend<HeaderValue>>(&self, _values: &mut E) {
        todo!()
    }
}

impl TryFrom<&str> for Producer {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(anyhow!("invalid producer: {}", value));
        }

        Ok(Self(value.to_owned()))
    }
}
