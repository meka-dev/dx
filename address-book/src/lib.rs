use chainrs::cosmos::base::AccountId;
use lazy_static::lazy_static;
use serde::Deserialize;

const CURRENT_JSON: &str = include_str!("../library/current.json");

lazy_static! {
    pub static ref CURRENT: AddressBook = serde_json::from_str(CURRENT_JSON).unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddressBook {
    pub arby: AccountId,
    pub treasury: AccountId,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use chainrs::cosmos::base::AccountId;
    use serde_json::json;

    use super::AddressBook;

    #[test]
    fn json_roundtrip() {
        let example = json!({
            "arby": AccountId::from_str("osmo1qjm68rhpd2fs48js38jeszjujqzyfpwea9wyak").unwrap(),
            "treasury": AccountId::from_str("osmo1qjm68rhpd2fs48js38jeszjujqzyfpwea9wyak").unwrap(),
        });
        let example_json = example.to_string();

        let address_book = serde_json::from_str::<AddressBook>(&example_json).unwrap();

        assert_eq!(address_book, *super::CURRENT);
    }
}
