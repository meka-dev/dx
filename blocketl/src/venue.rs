use std::str::FromStr;

use common::{err, Report};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub enum Venue {
    Osmosis,
    Terra,
}

impl FromStr for Venue {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Osmosis" => Ok(Self::Osmosis),
            "Terra" => Ok(Self::Terra),
            _ => Err(err!("unknown venue: {}", s)),
        }
    }
}
