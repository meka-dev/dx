use std::str::FromStr;

use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::{Addr, Uint128};
use criterion::{criterion_group, criterion_main, Criterion};

use execution::strategy::multi_dex::utils::compute_cycle_liq;
use terrars::dex::{Asset, Pool, Token};

fn criterion_benchmark(c: &mut Criterion) {
    let pools = vec![
        Pool {
            assets: [
                Asset {
                    token: Token::Native {
                        denom: "uusd".to_owned(),
                    },
                    amount: Uint128::from(1_000_000_000u128),
                    amount_decimal: Decimal256::from_str("1000.000000").unwrap(),
                    decimals: 6,
                },
                Asset {
                    token: Token::Cw20 {
                        contract_addr: Addr::unchecked("foobar"),
                    },
                    amount: Uint128::from(500_000_000u128),
                    amount_decimal: Decimal256::from_str("500.000000").unwrap(),
                    decimals: 6,
                },
            ],
            contract_address: Addr::unchecked("bazbar"),
            liquidity_token: Addr::unchecked("bazbar"),
            total_share: Uint128::from(10u128),
        },
        Pool {
            assets: [
                Asset {
                    token: Token::Cw20 {
                        contract_addr: Addr::unchecked("baz"),
                    },
                    amount: Uint128::from(2_000_000_000u128),
                    amount_decimal: Decimal256::from_str("1000.000000").unwrap(),
                    decimals: 6,
                },
                Asset {
                    token: Token::Cw20 {
                        contract_addr: Addr::unchecked("foobar"),
                    },
                    amount: Uint128::from(500_000_000u128),
                    amount_decimal: Decimal256::from_str("500.000000").unwrap(),
                    decimals: 6,
                },
            ],
            contract_address: Addr::unchecked("bazbar"),
            liquidity_token: Addr::unchecked("bazbar"),
            total_share: Uint128::from(10u128),
        },
        Pool {
            assets: [
                Asset {
                    token: Token::Cw20 {
                        contract_addr: Addr::unchecked("baz"),
                    },
                    amount: Uint128::from(2_000_000_000u128),
                    amount_decimal: Decimal256::from_str("1000.000000").unwrap(),
                    decimals: 6,
                },
                Asset {
                    token: Token::Cw20 {
                        contract_addr: Addr::unchecked("snafu"),
                    },
                    amount: Uint128::from(500_000_000u128),
                    amount_decimal: Decimal256::from_str("500.000000").unwrap(),
                    decimals: 6,
                },
            ],
            contract_address: Addr::unchecked("bazbar"),
            liquidity_token: Addr::unchecked("bazbar"),
            total_share: Uint128::from(10u128),
        },
        Pool {
            assets: [
                Asset {
                    token: Token::Native {
                        denom: "uusd".to_owned(),
                    },
                    amount: Uint128::from(1_000_000_000u128),
                    amount_decimal: Decimal256::from_str("1000.000000").unwrap(),
                    decimals: 6,
                },
                Asset {
                    token: Token::Cw20 {
                        contract_addr: Addr::unchecked("snafu"),
                    },
                    amount: Uint128::from(500_000_000u128),
                    amount_decimal: Decimal256::from_str("500.000000").unwrap(),
                    decimals: 6,
                },
            ],
            contract_address: Addr::unchecked("bazbar"),
            liquidity_token: Addr::unchecked("bazbar"),
            total_share: Uint128::from(10u128),
        },
    ];
    let start = Token::Native {
        denom: "uusd".to_owned(),
    };
    c.bench_function("compute_cycle_liq", |b| {
        b.iter(|| compute_cycle_liq(&start, pools.iter()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
