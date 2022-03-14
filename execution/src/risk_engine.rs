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
            Addr::unchecked("terra1m9hm6cxlf0907yy7lsssyfpzswlu54r99k9dxf"), /* LUART token has a transfer fee */
            Addr::unchecked("terra143az0w2e504n56q7k43qyh2fu69fh3rhup32n3"),
            Addr::unchecked("terra1ht8f0pnym9tcnt5srxmlnyndsfl84nntphqfsl"), // ???
            Addr::unchecked("terra136vrkx8kq7x5w0kgklzh9vxmn2x93q0edqe3nf"), // ???
            Addr::unchecked("terra1uyvksaz9xpgxpqnq8vz9vzg59up8mlhx3463tf"), // transfer fee
            Addr::unchecked("terra1ffjjxeu5l9f027qtkwseulgz6ddx5f04czyf0w"), // transfer fee
            Addr::unchecked("terra1t0pd5uqsqp85cgly5cvggm698asez55ked2ftu"), /* ALTE-MIR low liq, constant rounding errors? */
            Addr::unchecked("terra15asyerx0su2j3urlsn2xea0klrr5dpd75jamwj"), /* ALTE-SPEC low liq, constant rounding errors? */
            Addr::unchecked("terra1r0288hx3mjgx0zrpmpc8rulm6ca8jheqtwhwq5"), /* ALTE-STT low liq, constant rounding errors? */
            Addr::unchecked("terra1p2462m98je6pk3n03fwrnmngkm05mcr9pn6cmt"), /* ALTE-EUR low liq, constant rounding errors? */
            Addr::unchecked("terra1hkp4hulmttwzagzphr6vczvjm46e6ryxv99nan"), /* ALTE-MINE low liq, constant rounding errors? */
            Addr::unchecked("terra16eecm8exp3p82ld5eh5feweup0qvfv8468np6j"), /* ALTE-TWD low liq, constant rounding errors? */
            Addr::unchecked("terra1kh2g4fnhvqtnwwpqa84eywn72ve9vdkp5chhlx"), /* ALTE-LUNA low liq, constant rounding errors? */
            Addr::unchecked("terra1alaaant5ykususyuv0x6k298sh8kl8qkpkuu8e"), /* ALTE-aUST low liq, constant rounding errors? */
            Addr::unchecked("terra10dtff8drpwtfx4976m7cxwpptfy87l5p264xgy"), /* ALTE-ANC low liq, constant rounding errors? */
        ]);

        Self(h)
    }
}

impl RiskClassifier for StaticPoolDenyList {
    fn exclude(&self, item: &Pool) -> bool {
        self.0.contains(&item.contract_address)
    }
}

#[derive(Default)]
pub struct UnsupportedPoolList(pub HashSet<Addr>);

impl UnsupportedPoolList {
    pub fn new() -> Self {
        let h: HashSet<Addr> = HashSet::from([
            // Stableswap pools
            Addr::unchecked("terra1wgdjvp388mlvhad8u7ly5d34ga4zyyfvf3e5j8"),
            Addr::unchecked("terra1r5m5h9nvnw0hhzwn4gzz9p2wzg78q2jsq0alvp"),
            Addr::unchecked("terra17umn0xctu4qj8w6psqfjfp20nx454csspuvdur"),
            Addr::unchecked("terra1h2g2mldq2fskq0sqdhupnzpfj396d03ds2yfkd"),
            Addr::unchecked("terra1qswfc7hmmsnwf7f2nyyx843sug60urnqgz75zu"),
            Addr::unchecked("terra1gxjjrer8mywt4020xdl5e5x7n6ncn6w38gjzae"),
            Addr::unchecked("terra1cevdyd0gvta3h79uh5t47kk235rvn42gzf0450"),
            Addr::unchecked("terra1szt6cq52akhmzcqw5jhkw3tvdjtl4kvyk3zkhx"),
            Addr::unchecked("terra1qmxkqcgcgq8ch72k6kwu3ztz6fh8tx2xd76ws7"),
            Addr::unchecked("terra1cc6kqk0yl25hdpr5llxmx62mlyfdl7n0rwl3hq"),
            Addr::unchecked("terra1x0ulpvp6m46c5j7t40nj24mjp900954ys2jsnu"),
            Addr::unchecked("terra1t59ywlkkmvy7rmkdmdpy0z7wdqez5tne9mrd0r"),
            Addr::unchecked("terra1j66jatn3k50hjtg2xemnjm8s7y8dws9xqa5y8w"),
            // ibc assets
            Addr::unchecked("terra1tq4mammgkqrxrmcfhwdz59mwvwf4qgy6rdrt46"),
            Addr::unchecked("terra194j4rhezku4dmfz2vsnpzqv0actlg3xepg7l3t"),
        ]);

        Self(h)
    }
}

impl RiskClassifier for UnsupportedPoolList {
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
