use chainrs::cosmos::base::AccountId;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

const CURRENT_JSON: &str = include_str!("../library/current.json");

lazy_static! {
    pub static ref CURRENT: AddressBook = serde_json::from_str(CURRENT_JSON).unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddressBook {
    pub arby: AccountId,
    pub sprinters: Vec<Sprinter>,
    pub treasury: AccountId,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "state")]
pub enum Feegrant {
    Granted { host: String },
    Revoked { last_host: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Sprinter {
    pub account_id: AccountId,
    pub feegrant: Feegrant,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use chainrs::cosmos::base::AccountId;
    use serde_json::json;

    use super::{AddressBook, Feegrant};

    #[test]
    fn json_roundtrip() {
        let example = json!({
            "arby": AccountId::from_str("terra1lerw0sk4zt4nkxqr465ekgflpl4e98kl3ydm58").unwrap(),
            "treasury": AccountId::from_str("terra1sstgnc5rvmhte5rju9vh75wgnf3dktavmawzrp").unwrap(),
            "sprinters": [
                {
                    "account_id": AccountId::from_str("terra175yp3rnjk2av64zlkt777kkqa7f829g2hfpn4l").unwrap(),
                    "feegrant": Feegrant::Revoked { last_host: "exe1".to_string() }
                },
                {
                    "account_id": AccountId::from_str("terra1tzflepkdtn7gfa9quwv8qtn6zc4yqc0zxlxyr3").unwrap(),
                    "feegrant": Feegrant::Revoked { last_host: "exe2".to_string() }
                },
                {
                    "account_id": AccountId::from_str("terra12xcq4qs0lnrfnh4ulx6gngnj886dckqxeag258").unwrap(),
                    "feegrant": Feegrant::Granted { host: "exe3".to_string() }
                },
            ],
        });
        let example_json = example.to_string();

        let address_book = serde_json::from_str::<AddressBook>(&example_json).unwrap();

        assert_eq!(address_book, *super::CURRENT);
    }
}
