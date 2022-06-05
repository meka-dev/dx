use core::fmt;

use prometheus_client::encoding::text::Encode;

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Module {
    MultiDex,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::MultiDex => write!(f, "MultiDex"),
        }
    }
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Name {
    SinglePair,
    MultiHop,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::SinglePair => write!(f, "SinglePair"),
            Self::MultiHop => write!(f, "MultiHop"),
        }
    }
}

pub trait Labeled {
    const MODULE: Module;
    const NAME: Name;
}
