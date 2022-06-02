mod asset;
pub use asset::Asset;
mod pool;
pub use pool::{Pool, Pools};
mod token;
pub use token::{Pair as TokenPair, Token};

pub mod astroport {
    pub const ADDRESS: &str = "terra1fnywlw4edny3vw44x04xd67uzkdqluymgreu7g";
}

pub mod loopmarkets {
    pub const ADDRESS: &str = "terra16hdjuvghcumu6prg22cdjl96ptuay6r0hc6yns";
}

pub mod terraswap {
    pub const ADDRESS: &str = "terra1ulgw0td86nvs4wtpsc80thv6xelk76ut7a7apj";
}
