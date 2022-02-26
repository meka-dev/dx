use std::collections::HashSet;

use cosmwasm_std::{Addr, Uint128};

use terrars::dex::Pool;

pub struct RiskEngine<M>
where
    M: RiskClassifier + ?Sized,
{
    classifiers: Vec<Box<M>>,
}

impl<M> RiskEngine<M>
where
    M: RiskClassifier + ?Sized,
{
    pub fn new(classifiers: Vec<Box<M>>) -> RiskEngine<M> {
        RiskEngine { classifiers }
    }

    pub fn exclude(&self, pool: &Pool) -> bool {
        self.classifiers.iter().any(|cls| cls.exclude(pool))
    }
}

pub trait RiskClassifier: Send + Sync {
    fn exclude(&self, item: &Pool) -> bool;
}

#[derive(Default)]
pub struct EmptyPoolDeny {}

impl RiskClassifier for EmptyPoolDeny {
    fn exclude(&self, item: &Pool) -> bool {
        item.total_share == Uint128::zero()
            || item.assets[0].amount == Uint128::zero()
            || item.assets[1].amount == Uint128::zero()
    }
}

#[derive(Default)]
pub struct StaticPoolDenyList(pub HashSet<Addr>);

impl StaticPoolDenyList {
    pub fn new() -> Self {
        let h: HashSet<Addr> = HashSet::from([
            Addr::unchecked("terra1gs8pm90pllcz5t7hsqs2fnxjv6pxzf47gg3eua"),
            Addr::unchecked("terra1rhk92dvz3tjayymy8pl08gpkmamnud7ttzc03h"),
            Addr::unchecked("terra1jwl5rzg3xe0a3f8r26l0l3d957vlxm833srtcp"),
            Addr::unchecked("terra1ng5xg5p63c3qvjpwqsmry2dz8mr9n39v8eztv9"),
            Addr::unchecked("terra1z3u6mw3s20kt67ts9txrg2mjkzcwt7tgasp7gw"),
            Addr::unchecked("terra1lnr6aacxfng34m69076s2mdfjzt8nev2p6z5q0"),
            Addr::unchecked("terra1nf6wl37jxthskggvesngsp7y50s0jchmur57v3"),
        ]);

        Self(h)
    }
}

impl RiskClassifier for StaticPoolDenyList {
    fn exclude(&self, item: &Pool) -> bool {
        self.0.contains(&item.contract_address)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, str::FromStr as _};

    use cosmwasm_bignumber::Decimal256;
    use cosmwasm_std::{Addr, Uint128};
    use terrars::{
        dex::{Asset, Pool, Token},
        lcd::{self, Lcd},
    };

    use super::*;

    fn mkpool<L: Lcd>(addr: Addr) -> Pool {
        let assets = [
            Asset {
                token: Token::Native {
                    denom: "uluna".to_owned(),
                },
                amount: Uint128::new(1000000u128),
                amount_decimal: Decimal256::from_str("1.0").unwrap(),
                decimals: 6,
            },
            Asset {
                token: Token::Native {
                    denom: "uusd".to_owned(),
                },
                amount: Uint128::new(1000000u128),
                amount_decimal: Decimal256::from_str("1.0").unwrap(),
                decimals: 6,
            },
        ];
        Pool {
            contract_address: addr,
            assets,
            liquidity_token: Addr::unchecked("foobar"),
            total_share: Uint128::new(1u128),
        }
    }

    #[test]
    fn static_deny_list() {
        let h = HashSet::from([
            Addr::unchecked("terra1gs8pm90pllcz5t7hsqs2fnxjv6pxzf47gg3eua"),
            Addr::unchecked("terra1rhk92dvz3tjayymy8pl08gpkmamnud7ttzc03h"),
        ]);
        let dl = StaticPoolDenyList(h);
        let pool = mkpool::<lcd::Http>(Addr::unchecked(
            "terra1gs8pm90pllcz5t7hsqs2fnxjv6pxzf47gg3eua",
        ));
        assert!(dl.exclude(&pool));

        let pool = mkpool::<lcd::Http>(Addr::unchecked(
            "terra1yphapr9zkd7eq60n2nux0xfldzs58z9ncs7s6f",
        ));
        assert!(!dl.exclude(&pool));
    }
}
