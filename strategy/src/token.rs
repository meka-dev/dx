#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Token {
    pub id: String, // Denom?
    pub decimals: u8,
    pub native: bool,
}
