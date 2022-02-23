use std::{collections::HashMap, marker::PhantomData};

use cosmwasm_std::Addr;

use terrars::{dex::Pool, lcd::Lcd};

pub struct RiskEngine<L, M>
where
    M: RiskClassifier<L> + ?Sized,
{
    classifiers: Vec<Box<M>>,
    _lcd: PhantomData<L>,
}

impl<L, M> RiskEngine<L, M>
where
    M: RiskClassifier<L> + ?Sized,
{
    pub fn new(classifiers: Vec<Box<M>>) -> RiskEngine<L, M> {
        RiskEngine {
            classifiers,
            _lcd: PhantomData,
        }
    }

    pub fn exclude(&self, pool: &dyn Pool<L>) -> bool {
        self.classifiers.iter().any(|cls| cls.exclude(pool))
    }
}

pub trait RiskClassifier<L>: Send + Sync {
    fn exclude(&self, item: &dyn Pool<L>) -> bool;
}

#[derive(Default)]
pub struct StaticPoolDenyList(pub HashMap<Addr, bool>);

impl StaticPoolDenyList {
    pub fn new() -> Box<Self> {
        let h: HashMap<Addr, bool> = HashMap::from([
            (
                Addr::unchecked("terra1gs8pm90pllcz5t7hsqs2fnxjv6pxzf47gg3eua"),
                true,
            ),
            (
                Addr::unchecked("terra1rhk92dvz3tjayymy8pl08gpkmamnud7ttzc03h"),
                true,
            ),
            (
                Addr::unchecked("terra1jwl5rzg3xe0a3f8r26l0l3d957vlxm833srtcp"),
                true,
            ),
            (
                Addr::unchecked("terra1ng5xg5p63c3qvjpwqsmry2dz8mr9n39v8eztv9"),
                true,
            ),
            (
                Addr::unchecked("terra1z3u6mw3s20kt67ts9txrg2mjkzcwt7tgasp7gw"),
                true,
            ),
            (
                Addr::unchecked("terra1lnr6aacxfng34m69076s2mdfjzt8nev2p6z5q0"),
                true,
            ),
            (
                Addr::unchecked("terra1nf6wl37jxthskggvesngsp7y50s0jchmur57v3"),
                true,
            ),
        ]);

        Box::new(StaticPoolDenyList(h))
    }
}

impl<L: Lcd> RiskClassifier<L> for StaticPoolDenyList {
    fn exclude(&self, item: &dyn Pool<L>) -> bool {
        self.0.contains_key(&item.addr())
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, str::FromStr};

    use cosmwasm_bignumber::Decimal256;
    use cosmwasm_std::{Addr, Uint128};
    use terrars::{
        dex::{terraswap::Pair, Asset, Token},
        lcd::{self, Lcd},
    };

    use super::*;

    fn mkpool<L: Lcd>(addr: Addr) -> impl Pool<L> {
        let coins = [
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
        Pair {
            contract_addr: addr,
            coins,
            liquidity_token: "foobar".to_owned(),
            total_share: Uint128::new(1u128),
        }
    }

    #[test]
    fn static_deny_list() {
        let h = HashMap::from([
            (
                Addr::unchecked("terra1gs8pm90pllcz5t7hsqs2fnxjv6pxzf47gg3eua"),
                true,
            ),
            (
                Addr::unchecked("terra1rhk92dvz3tjayymy8pl08gpkmamnud7ttzc03h"),
                true,
            ),
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
