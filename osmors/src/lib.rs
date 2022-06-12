use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::Deserialize;

const CURRENT_JSON: &str = include_str!("../assets.json");

lazy_static! {
    pub static ref ASSET_DECIMALS: HashMap<String, u8> = {
        let am: Vec<AssetMetaData> = serde_json::from_str(CURRENT_JSON).unwrap();
        am.into_iter().map(|a| (a.denom, a.decimals)).collect()
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssetMetaData {
    pub denom: String,
    pub decimals: u8,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::AssetMetaData;

    #[test]
    fn json_roundtrip() {
        let example = json!(
        [
          {
            "decimals": 6u8,
            "denom": "uosmo",
            "name": "Osmosis"
          },
          {
            "decimals": 6u8,
            "denom": "uion",
            "name": "Ion"
          },
          {
            "decimals": 6u8,
            "denom": "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
            "name": "Cosmos"
          },
          {
            "decimals": 6u8,
            "denom": "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4",
            "name": "Akash Network"
          },
          {
            "decimals": 6u8,
            "denom": "ibc/A0CC0CF735BFB30E730C70019D4218A1244FF383503FF7579C9201AB93CA9293",
            "name": "Persistence"
          },
          {
            "decimals": 18u8,
            "denom": "ibc/8061A06D3BD4D52C4A28FFECF7150D370393AF0BA661C3776C54FF32836C3961",
            "name": "PSTAKE"
          }]
                    );
        let example_json = example.to_string();

        let mds: HashMap<String, u8> = serde_json::from_str::<Vec<AssetMetaData>>(&example_json)
            .unwrap()
            .into_iter()
            .map(|a| (a.denom, a.decimals))
            .collect();

        assert_eq!(
            mds.get("uosmo").unwrap(),
            super::ASSET_DECIMALS.get("uosmo").unwrap()
        );
    }
}
