use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

use terra_rust_api::Terra;
use terraswap::{asset, factory, pair};

const TERRASWAP_ADDRESS: &str = "terra1ulgw0td86nvs4wtpsc80thv6xelk76ut7a7apj";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct PairsResult {
    height: String,
    result: factory::PairsResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct PairInfoResult {
    height: String,
    result: asset::PairInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct PoolResponseResult {
    height: String,
    result: pair::PoolResponse,
}

#[derive(Debug)]
pub struct Pair {
    pub contract_addr: String,
    pub coins: [asset::Asset; 2],
    pub liquidity_token: String,
    pub total_share: Uint128,
}

pub async fn liquid_pairs(
    lcd: &Terra,
    filter: Option<asset::AssetInfo>,
) -> anyhow::Result<Vec<Pair>> {
    let ps = pairs(lcd).await?.into_iter().filter(|pair| {
        if let Some(coin) = filter.clone() {
            pair.asset_infos[0].equal(&coin) || pair.asset_infos[1].equal(&coin)
        } else {
            true
        }
    });

    let mut pairs: Vec<Pair> = vec![];
    for pair in ps {
        println!("{}", pair.contract_addr);

        match pool_info(lcd, &pair.contract_addr).await {
            Ok(pool) => {
                if pool.total_share == Uint128::zero()
                    || pool.assets[0].amount == Uint128::zero()
                    || pool.assets[1].amount == Uint128::zero()
                {
                    continue;
                }

                pairs.push(Pair {
                    contract_addr: pair.contract_addr.clone(),
                    coins: pool.assets,
                    liquidity_token: pair.liquidity_token.clone(),
                    total_share: pool.total_share,
                });
            },
            Err(err) => {
                eprintln!("{}", err);
            },
        }
    }

    Ok(pairs)
}

pub async fn update_pairs(lcd: &Terra, pairs: &mut Vec<Pair>) -> anyhow::Result<()> {
    // TODO(xla): Parallelise with batches modulo num cpus.
    for pair in pairs {
        let pool = pool_info(lcd, &pair.contract_addr).await?;

        pair.coins = pool.assets;
        pair.total_share = pool.total_share;
    }

    Ok(())
}

pub async fn pairs(lcd: &Terra) -> anyhow::Result<Vec<asset::PairInfo>> {
    let mut pairs: Vec<asset::PairInfo> = vec![];
    let mut start_after: Option<[asset::AssetInfo; 2]> = None;
    let limit = 30;

    loop {
        let mut res = lcd
            .wasm()
            .query::<PairsResult>(
                TERRASWAP_ADDRESS,
                &serde_json::to_string(&factory::QueryMsg::Pairs {
                    start_after,
                    limit: Some(30),
                })?,
            )
            .await?;

        start_after = res.result.pairs.last().map(|pair| pair.asset_infos.clone());
        let size = res.result.pairs.len();

        pairs.append(&mut res.result.pairs);

        if size < limit {
            break;
        }
    }

    Ok(pairs)
}

#[allow(dead_code)]
pub async fn pair_info(lcd: &Terra, contract_addr: &str) -> anyhow::Result<asset::PairInfo> {
    let res = lcd
        .wasm()
        .query::<PairInfoResult>(
            contract_addr,
            &serde_json::to_string(&pair::QueryMsg::Pair {})?,
        )
        .await?;

    Ok(res.result)
}

pub async fn pool_info(
    lcd: &Terra,
    contract_addr: &str,
) -> anyhow::Result<terraswap::pair::PoolResponse> {
    let res = lcd
        .wasm()
        .query::<PoolResponseResult>(
            contract_addr,
            &serde_json::to_string(&terraswap::pair::QueryMsg::Pool {})?,
        )
        .await?;

    Ok(res.result)
}
