use std::ops;

use common::{err, Report};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Namespace(String);

impl ops::Deref for Namespace {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for Namespace {
    type Error = Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(err!("invalid producer: {}", value));
        }

        Ok(Self(value))
    }
}
