use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default)]
pub struct Token {
    pub id: String, // Denom?
    pub decimals: u8,
    pub native: bool,
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.decimals == other.decimals
    }
}

impl Eq for Token {}
