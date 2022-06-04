use std::str::FromStr;

use cosmwasm_bignumber::{Decimal256, Uint256};
use criterion::{criterion_group, criterion_main, Criterion};
use strategy::{xyk::compute_cycle_liq, Pool, Token, WeightedAsset};

fn criterion_benchmark(c: &mut Criterion) {
    let pools = vec![
        Pool {
            assets: [
                WeightedAsset {
                    token: Token {
                        id: "uusd".to_owned(),
                        decimals: 6,
                        native: true,
                    },
                    amount: Decimal256::from_str("100000.0").unwrap(),
                    weight: Uint256::from(100u128),
                },
                WeightedAsset {
                    token: Token {
                        id: "foobar".to_owned(),
                        decimals: 6,
                        native: false,
                    },
                    amount: Decimal256::from_str("500000.0").unwrap(),
                    weight: Uint256::from(100u128),
                },
            ],
            id: "bazbar".to_owned(),
            swap_fee_bid: Decimal256::permille(1000),
            swap_fee_ask: Decimal256::permille(997),
        },
        Pool {
            assets: [
                WeightedAsset {
                    token: Token {
                        id: "baz".to_owned(),
                        decimals: 6,
                        native: false,
                    },
                    amount: Decimal256::from_str("200000.0").unwrap(),
                    weight: Uint256::from(100u128),
                },
                WeightedAsset {
                    token: Token {
                        id: "foobar".to_owned(),
                        decimals: 6,
                        native: false,
                    },
                    amount: Decimal256::from_str("5000000.0").unwrap(),
                    weight: Uint256::from(100u128),
                },
            ],
            id: "bazbar".to_owned(),
            swap_fee_bid: Decimal256::permille(1000),
            swap_fee_ask: Decimal256::permille(997),
        },
        Pool {
            assets: [
                WeightedAsset {
                    token: Token {
                        id: "baz".to_owned(),
                        decimals: 6,
                        native: false,
                    },
                    amount: Decimal256::from_str("200000.0").unwrap(),
                    weight: Uint256::from(100u128),
                },
                WeightedAsset {
                    token: Token {
                        id: "snafu".to_owned(),
                        decimals: 6,
                        native: false,
                    },
                    amount: Decimal256::from_str("50000000.0").unwrap(),
                    weight: Uint256::from(100u128),
                },
            ],
            id: "bazbar".to_owned(),
            swap_fee_bid: Decimal256::permille(1000),
            swap_fee_ask: Decimal256::permille(997),
        },
        Pool {
            assets: [
                WeightedAsset {
                    token: Token {
                        id: "uusd".to_owned(),
                        decimals: 6,
                        native: true,
                    },
                    amount: Decimal256::from_str("100000.0").unwrap(),
                    weight: Uint256::from(100u128),
                },
                WeightedAsset {
                    token: Token {
                        id: "snafu".to_owned(),
                        decimals: 6,
                        native: false,
                    },
                    amount: Decimal256::from_str("50000000.0").unwrap(),
                    weight: Uint256::from(100u128),
                },
            ],
            id: "bazbar".to_owned(),
            swap_fee_bid: Decimal256::permille(1000),
            swap_fee_ask: Decimal256::permille(997),
        },
    ];
    let start = Token {
        id: "uusd".to_owned(),
        decimals: 6,
        native: true,
    };
    c.bench_function("compute_cycle_liq", |b| {
        b.iter(|| compute_cycle_liq(&start, pools.iter()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
