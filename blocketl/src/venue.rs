use std::str::FromStr;

use anyhow::{anyhow, Error};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub enum Venue {
    Osmosis,
    Terra,
}

impl FromStr for Venue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Osmosis" => Ok(Self::Osmosis),
            "Terra" => Ok(Self::Terra),
            _ => Err(anyhow!("unknown venue: {}", s)),
        }
    }
}
