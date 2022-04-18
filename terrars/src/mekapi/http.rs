use anyhow::{Context as _, Result};
use async_trait::async_trait;
use cosmrs::tendermint::block;
use cosmwasm_std::Addr;
use futures::stream::{BoxStream, StreamExt as _};
use reqwest::{header::HeaderValue, Client, ClientBuilder, StatusCode, Url};
use reqwest_eventsource::{Event, RequestBuilderExt as _};
use tracing::debug;

use crate::dex::Pool;

const HEADER_COUNT: &str = "Mekatek-Count";
static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

use crate::mekapi::{v1, v3};

#[derive(Clone)]
pub struct Http {
    base_url: Url,
    client: Client,
}

impl Http {
    pub fn new(url: String) -> Result<Self> {
        let base_url = Url::parse(&url)?;
        let client = ClientBuilder::new().user_agent(USER_AGENT).build()?;

        Ok(Self { base_url, client })
    }
}

#[async_trait]
impl v1::Api for Http {
    async fn v1_events_mempooltx(
        &self,
    ) -> Result<BoxStream<'static, Result<v1::MempoolTxMessage>>> {
        let url = self.base_url.join("v1/events/mempooltx-v1")?;
        let sse = self.client.get(url).eventsource()?;

        let tx_stream = sse
            .scan((), |_, res| async move {
                match res {
                    Err(reqwest_eventsource::Error::StreamEnded) => None,
                    res => Some(res),
                }
            })
            .filter_map(|res| async move {
                match res {
                    Ok(Event::Message(eventsource_stream::Event { event, data, .. }))
                        if event == "mempooltx-v1".to_string() =>
                    {
                        let res = serde_json::from_str::<v1::MempoolTxMessage>(&data)
                            .context("failed to decode tx event message data");

                        Some(res)
                    },
                    _ => None,
                }
            });

        Ok(tx_stream.fuse().boxed())
    }

    async fn v1_events_mempooltx_v2(
        &self,
    ) -> Result<BoxStream<'static, Result<v1::MempoolTxV2Message>>> {
        let url = self.base_url.join("v1/events/mempooltx-v2?only-pools")?;
        let sse = self.client.get(url).eventsource()?;

        let tx_stream = sse
            .scan((), |_, res| async move {
                match res {
                    Err(reqwest_eventsource::Error::StreamEnded) => None,
                    res => Some(res),
                }
            })
            .filter_map(|res| async move {
                match res {
                    Ok(Event::Message(eventsource_stream::Event { event, data, .. }))
                        if event == "mempooltx-v2".to_string() =>
                    {
                        let res = serde_json::from_str::<v1::MempoolTxV2Message>(&data)
                            .context("failed to decode tx event message data");

                        Some(res)
                    },
                    _ => None,
                }
            });

        Ok(tx_stream.fuse().boxed())
    }

    async fn v1_pool_responses(&self, factory: Addr, height: block::Height) -> Result<Vec<Pool>> {
        let mut url = self.base_url.join("v1/poolresponses")?;
        url.query_pairs_mut()
            .clear()
            .append_pair("factory", factory.as_ref())
            .append_pair("height", &height.to_string());

        let raw = self.client.get(url).send().await?;
        let pools = match raw.status() {
            StatusCode::OK => {
                let expected_len: usize = raw
                    .headers()
                    .get(HEADER_COUNT)
                    .unwrap_or(&HeaderValue::from_static("0"))
                    .to_str()?
                    .parse()?;

                let body = raw.text_with_charset("utf-8").await?;

                let pools = body
                    .lines()
                    .into_iter()
                    .filter_map(|val| hex::decode(val).ok())
                    .filter_map(|value| cosmwasm_std::from_slice::<Pool>(&value).ok())
                    .collect::<Vec<_>>();

                if pools.len() < expected_len {
                    debug!(target: "mekapi", "decoded less pools than sent: {} < {}",
                        pools.len(),
                        expected_len
                    );
                }

                pools
            },
            code => return Err(anyhow::anyhow!("unexpected response status code: {}", code)),
        };

        Ok(pools)
    }
}

#[async_trait]
impl v3::Api for Http {
    async fn v3_pools(&self, factory: Addr, height: block::Height) -> Result<Vec<v3::Pool>> {
        let mut url = self.base_url.join("v3/pools")?;
        url.query_pairs_mut()
            .clear()
            .append_pair("factory", factory.as_ref())
            .append_pair("height", &height.to_string());

        let res = self.client.get(url).send().await?;
        if res.status() != StatusCode::OK {
            return Err(anyhow::anyhow!(
                "unexpected response status code: {}",
                res.status()
            ));
        }

        let res = res.json::<v3::PoolsResponse>().await?;

        if res.factory != factory {
            return Err(anyhow::anyhow!(
                "returned factory doesn't match input: {} != {}",
                res.factory,
                factory
            ));
        }

        if res.height != height {
            return Err(anyhow::anyhow!(
                "returned height doesn't match input: {} != {}",
                res.height,
                height
            ));
        }

        Ok(res.pools)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use cosmrs::{tendermint::block, Denom};
    use cosmwasm_bignumber::Decimal256;
    use cosmwasm_std::{Addr, Uint128};
    use futures::TryStreamExt as _;
    use pretty_assertions::assert_eq;
    use wiremock::{
        matchers::{method, path, query_param},
        Mock,
        MockServer,
        ResponseTemplate,
    };

    use super::{
        v1::Api as _,
        v3::{self, Api as _},
        Http,
    };
    use crate::dex::{self, Token};

    #[tokio::test]
    async fn v1_events_mempooltx() -> Result<()> {
        let response = ResponseTemplate::new(200).set_body_raw(
            include_bytes!("../../fixtures/mekapi/v1_events_mempooltx.eventstream").to_vec(),
            "text/event-stream",
        );

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("v1/events/mempooltx-v1"))
            .respond_with(response)
            .mount(&server)
            .await;

        let http = Http::new(server.uri())?;
        let msgs = http
            .v1_events_mempooltx()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        assert_eq!(msgs.len(), 15);

        Ok(())
    }

    #[tokio::test]
    async fn v1_events_mempooltx_v2() -> Result<()> {
        let response = ResponseTemplate::new(200).set_body_raw(
            include_bytes!("../../fixtures/mekapi/v1_events_mempooltx_v2.eventstream").to_vec(),
            "text/event-stream",
        );

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("v1/events/mempooltx-v2"))
            .respond_with(response)
            .mount(&server)
            .await;

        let http = Http::new(server.uri())?;
        let msgs = http
            .v1_events_mempooltx_v2()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        assert_eq!(msgs.len(), 9);

        Ok(())
    }

    #[tokio::test]
    async fn v1_pool_responses_astroport() -> Result<()> {
        let factory = Addr::unchecked("terra1fnywlw4edny3vw44x04xd67uzkdqluymgreu7g");
        let height = block::Height::from_str("1111")?;

        let server = MockServer::start().await;
        let response = ResponseTemplate::new(200).set_body_raw(r#"7b22636f6e74726163745f61646472657373223a22746572726131717075397665376c6c6539776c376e706b71713734356a386677656b77356768666e74746b6a222c226c69717569646974795f746f6b656e223a227465727261317a656a796a70776636667065673766706134327675306435666a77656a76377478347473337a222c22617373657473223a5b7b22696e666f223a7b22746f6b656e223a7b226e616d65223a22464f5820546f6b656e222c22636f6e74726163745f61646472223a2274657272613137636b7832356b7168777a663366756470727337386b716770336c6870723277373479663774227d7d2c22616d6f756e74223a2230222c22746f74616c5f737570706c79223a2232303030303030303030303030303030303030227d2c7b22696e666f223a7b226e61746976655f746f6b656e223a7b2264656e6f6d223a2275757364227d7d2c22616d6f756e74223a2230222c22746f74616c5f737570706c79223a223132303730363134343334363633383433227d5d2c22746f74616c5f7368617265223a2230227d
7b22636f6e74726163745f61646472657373223a227465727261317172326b36796a6a643570326b6165777176673933616737346b3667796a7237726533376673222c226c69717569646974795f746f6b656e223a22746572726131776d61747936357974376d6a7736666a66796d6b64397a736d36617473713832643961726364222c22617373657473223a5b7b22696e666f223a7b22746f6b656e223a7b226e616d65223a22416e63686f7220546f6b656e222c22636f6e74726163745f61646472223a22746572726131347a35366c306670326c736638367a7933687479327a3437657a6b686e746874723979713736227d7d2c22616d6f756e74223a223339343636343233313031303535222c22746f74616c5f737570706c79223a2231303030303030303030303030303030227d2c7b22696e666f223a7b226e61746976655f746f6b656e223a7b2264656e6f6d223a2275757364227d7d2c22616d6f756e74223a223839343831313736373735383238222c22746f74616c5f737570706c79223a223132303730363134343334363633383433227d5d2c22746f74616c5f7368617265223a223539303631343139383537353637227d
7b22636f6e74726163745f61646472657373223a227465727261317176306a35756467326c636e716b757935727174326e366e327a396d327a7073367a66783276222c226c69717569646974795f746f6b656e223a22746572726131753564767070747068677134337167766a396367376365386133676c36716863706c366a7866222c22617373657473223a5b7b22696e666f223a7b22746f6b656e223a7b226e616d65223a22507269736d2050726f746f636f6c20546f6b656e222c22636f6e74726163745f61646472223a227465727261316468393437386b32717671687165616a686e37356132613764736e6637347935756b72656777227d7d2c22616d6f756e74223a2230222c22746f74616c5f737570706c79223a2231303030303030303030303030303030227d2c7b22696e666f223a7b22746f6b656e223a7b226e616d65223a225465727261525820546f6b656e222c22636f6e74726163745f61646472223a22746572726131386b79396c6e34673678336e3966397338613574747368796e76366c677a6670706d726d7967227d7d2c22616d6f756e74223a2230222c22746f74616c5f737570706c79223a2232303030303030303030303030303030303030227d5d2c22746f74616c5f7368617265223a2230227d"#, "text/plain; charset=utf-8");
        Mock::given(method("GET"))
            .and(path("/v1/poolresponses"))
            .and(query_param("factory", &factory.to_string()))
            .and(query_param("height", &height.to_string()))
            .respond_with(response)
            .mount(&server)
            .await;

        let http = Http::new(server.uri())?;
        let pools = http.v1_pool_responses(factory, height).await?;

        assert_eq!(pools.len(), 3);
        assert_eq!(
            pools,
            vec![
                dex::Pool {
                    assets: [
                        dex::Asset {
                            amount: Uint128::zero(),
                            amount_decimal: Decimal256::zero(),
                            decimals: 6,
                            token: Token::Cw20 {
                                contract_addr: Addr::unchecked(
                                    "terra17ckx25kqhwzf3fudprs78kqgp3lhpr2w74yf7t"
                                ),
                            },
                        },
                        dex::Asset {
                            amount: Uint128::zero(),
                            amount_decimal: Decimal256::zero(),
                            decimals: 6,
                            token: Token::ust(),
                        },
                    ],
                    contract_address: Addr::unchecked(
                        "terra1qpu9ve7lle9wl7npkqq745j8fwekw5ghfnttkj"
                    ),
                    liquidity_token: Addr::unchecked(
                        "terra1zejyjpwf6fpeg7fpa42vu0d5fjwejv7tx4ts3z"
                    ),
                    total_share: Uint128::zero(),
                },
                dex::Pool {
                    assets: [
                        dex::Asset {
                            amount: Uint128::from(39466423101055u64),
                            amount_decimal: Decimal256::from_uint256(39466423101055u64)
                                / Decimal256::from_uint256(10u64.pow(6)),
                            decimals: 6,
                            token: Token::Cw20 {
                                contract_addr: Addr::unchecked(
                                    "terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76"
                                ),
                            },
                        },
                        dex::Asset {
                            amount: Uint128::from(89481176775828u64),
                            amount_decimal: Decimal256::from_uint256(89481176775828u64)
                                / Decimal256::from_uint256(10u64.pow(6)),
                            decimals: 6,
                            token: Token::ust(),
                        },
                    ],
                    contract_address: Addr::unchecked(
                        "terra1qr2k6yjjd5p2kaewqvg93ag74k6gyjr7re37fs"
                    ),
                    liquidity_token: Addr::unchecked(
                        "terra1wmaty65yt7mjw6fjfymkd9zsm6atsq82d9arcd"
                    ),
                    total_share: Uint128::from(59061419857567u64),
                },
                dex::Pool {
                    assets: [
                        dex::Asset {
                            amount: Uint128::zero(),
                            amount_decimal: Decimal256::zero(),
                            decimals: 6,
                            token: Token::Cw20 {
                                contract_addr: Addr::unchecked(
                                    "terra1dh9478k2qvqhqeajhn75a2a7dsnf74y5ukregw"
                                ),
                            },
                        },
                        dex::Asset {
                            amount: Uint128::zero(),
                            amount_decimal: Decimal256::zero(),
                            decimals: 6,
                            token: Token::Cw20 {
                                contract_addr: Addr::unchecked(
                                    "terra18ky9ln4g6x3n9f9s8a5ttshynv6lgzfppmrmyg"
                                ),
                            },
                        },
                    ],
                    contract_address: Addr::unchecked(
                        "terra1qv0j5udg2lcnqkuy5rqt2n6n2z9m2zps6zfx2v"
                    ),
                    liquidity_token: Addr::unchecked(
                        "terra1u5dvpptphgq43qgvj9cg7ce8a3gl6qhcpl6jxf"
                    ),
                    total_share: Uint128::zero(),
                },
            ]
        );

        Ok(())
    }

    #[tokio::test]
    async fn v1_pool_responses_loopmarkets() -> Result<()> {
        let factory = Addr::unchecked("terra16hdjuvghcumu6prg22cdjl96ptuay6r0hc6yns");
        let height = block::Height::from_str("1111")?;

        let server = MockServer::start().await;
        let response = ResponseTemplate::new(200).set_body_raw(r#"7b22636f6e74726163745f61646472657373223a22746572726131706d6d667a3230356573376178796d77706c346a6a327275786e657675667533343632663032222c226c69717569646974795f746f6b656e223a22746572726131617a7a716e3838773363633678386a776c326d61707573706a7a646b7a707a32367165326c6b222c22617373657473223a5b7b22696e666f223a7b22746f6b656e223a7b22636f6e74726163745f61646472223a2274657272613134746c3833786377716a79306b656e3970657534706a6a75753735356c727279327579323572227d7d2c22616d6f756e74223a22343331343337313439227d2c7b22696e666f223a7b226e61746976655f746f6b656e223a7b2264656e6f6d223a22756c756e61227d7d2c22616d6f756e74223a22323330333731363739227d5d2c22746f74616c5f7368617265223a223935393030313335227d
7b22636f6e74726163745f61646472657373223a227465727261317a37746b3430717a676d643272687a327663706c636b6761743033726c36743837677463666d222c226c69717569646974795f746f6b656e223a227465727261316172736c676c747179757a6d79703032777a35777268683861796579656c74747935666a7373222c22617373657473223a5b7b22696e666f223a7b22746f6b656e223a7b22636f6e74726163745f61646472223a227465727261316e6b756c746e776676767565336368306c667768736c30706e70337a64393733346c33753733227d7d2c22616d6f756e74223a2230227d2c7b22696e666f223a7b226e61746976655f746f6b656e223a7b2264656e6f6d223a2275757364227d7d2c22616d6f756e74223a2230227d5d2c22746f74616c5f7368617265223a2230227d
7b22636f6e74726163745f61646472657373223a22746572726131796a673074756863366b7a777a396a6c38797167786e66326374776c66756d6e767363757070222c226c69717569646974795f746f6b656e223a2274657272613137707a74387432686d78363538377a6e36796835656e73796c6d3373396d6d346d373276326e222c22617373657473223a5b7b22696e666f223a7b22746f6b656e223a7b22636f6e74726163745f61646472223a2274657272613177386b7664366371707374687570736b346c30636c776e6d656b346c337a72376338346b7771227d7d2c22616d6f756e74223a223230323339333739313633333433227d2c7b22696e666f223a7b226e61746976655f746f6b656e223a7b2264656e6f6d223a2275757364227d7d2c22616d6f756e74223a2231323035303237333038383231227d5d2c22746f74616c5f7368617265223a2234383832303738323633383730227d"#, "text/plain; charset=utf-8");
        Mock::given(method("GET"))
            .and(path("/v1/poolresponses"))
            .and(query_param("factory", &factory.to_string()))
            .and(query_param("height", &height.to_string()))
            .respond_with(response)
            .mount(&server)
            .await;

        let http = Http::new(server.uri())?;
        let pools = http.v1_pool_responses(factory, height).await?;

        assert_eq!(pools.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn v1_pool_responses_terraswap() -> Result<()> {
        let factory = Addr::unchecked("terra1ulgw0td86nvs4wtpsc80thv6xelk76ut7a7apj");
        let height = block::Height::from_str("1111")?;

        let server = MockServer::start().await;
        let response = ResponseTemplate::new(200).set_body_raw(r#"7b22636f6e74726163745f61646472657373223a227465727261317170703961726c777a30303839706c36307839796d616b6a6c637a393338386566756b787070222c226c69717569646974795f746f6b656e223a227465727261316334367937337a78616a78776b356c7a6530307a63653373657861666d7237666e6b71786867222c22617373657473223a5b7b22696e666f223a7b22746f6b656e223a7b22636f6e74726163745f61646472223a22746572726131746e703876677572637568746175307230716a7473783373776d723068686e366c3872343032227d7d2c22616d6f756e74223a2230227d2c7b22696e666f223a7b226e61746976655f746f6b656e223a7b2264656e6f6d223a2275757364227d7d2c22616d6f756e74223a2230227d5d2c22746f74616c5f7368617265223a2230227d
7b22636f6e74726163745f61646472657373223a22746572726131717064396e37616677663435726b6a6c70756a72726466683833706c646563387270756a676e222c226c69717569646974795f746f6b656e223a22746572726131716d72386a336d397835336468777330797868796d7a73766e6b6a71383836796b386b39336d222c22617373657473223a5b7b22696e666f223a7b22746f6b656e223a7b22636f6e74726163745f61646472223a227465727261316b6338376d7534363066776b71746532397271756834686332306d3534667877747378376770227d7d2c22616d6f756e74223a22333131323831343538227d2c7b22696e666f223a7b226e61746976655f746f6b656e223a7b2264656e6f6d223a2275757364227d7d2c22616d6f756e74223a223135363237333535373135227d5d2c22746f74616c5f7368617265223a2231393134343734393231227d
7b22636f6e74726163745f61646472657373223a227465727261317170366a70327366737638676d64683274753333773930347171796d333934667a6b786e7864222c226c69717569646974795f746f6b656e223a2274657272613133656b3978383768703974336766356c7463687a686c3378756868616b6d32366a796a34767a222c22617373657473223a5b7b22696e666f223a7b22746f6b656e223a7b22636f6e74726163745f61646472223a22746572726131756b6b3839676e3235703778736571633473346434357961726a6c7779636872777837756d76227d7d2c22616d6f756e74223a223130373233393137383333227d2c7b22696e666f223a7b226e61746976655f746f6b656e223a7b2264656e6f6d223a2275757364227d7d2c22616d6f756e74223a2231323635393238227d5d2c22746f74616c5f7368617265223a22313030303030303030227d"#, "text/plain; charset=utf-8");
        Mock::given(method("GET"))
            .and(path("/v1/poolresponses"))
            .and(query_param("factory", &factory.to_string()))
            .and(query_param("height", &height.to_string()))
            .respond_with(response)
            .mount(&server)
            .await;

        let http = Http::new(server.uri())?;
        let pools = http.v1_pool_responses(factory, height).await?;

        assert_eq!(pools.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn v3_pools_astroport() -> Result<()> {
        let factory = Addr::unchecked(dex::astroport::ADDRESS);
        let height = "6750103".parse::<block::Height>()?;
        let response = ResponseTemplate::new(200).set_body_bytes(
            include_bytes!("../../fixtures/mekapi/v3-pools_astroport.json").to_vec(),
        );

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("v3/pools"))
            .and(query_param("factory", &factory.to_string()))
            .and(query_param("height", &height.to_string()))
            .respond_with(response)
            .mount(&server)
            .await;

        let http = Http::new(server.uri())?;
        let pools = http.v3_pools(factory, height).await?;

        assert_eq!(pools.len(), 114);

        let anc_ust_addr = Addr::unchecked("terra1qr2k6yjjd5p2kaewqvg93ag74k6gyjr7re37fs");
        let anc_ust_pool = pools
            .iter()
            .find(|pool| pool.contract_address == anc_ust_addr)
            .unwrap();

        assert_eq!(anc_ust_pool.contract_address, anc_ust_addr);
        assert_eq!(
            anc_ust_pool.liquidity_token,
            Addr::unchecked("terra1wmaty65yt7mjw6fjfymkd9zsm6atsq82d9arcd")
        );
        assert_eq!(
            anc_ust_pool.total_share,
            Uint128::try_from(71152489166742u64)?
        );
        assert_eq!(
            anc_ust_pool.assets[0],
            v3::Asset::Cw20 {
                contract_addr: Addr::unchecked("terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76"),
                amount: Uint128::try_from(35440438044246u64)?,
                decimals: 6,
                name: "Anchor Token".to_string(),
                symbol: "ANC".to_string(),
                total_supply: Uint128::try_from(1000000000000000u64)?,
            },
        );
        assert_eq!(
            anc_ust_pool.assets[1],
            v3::Asset::Native {
                denom: "uusd".parse::<Denom>().unwrap(),
                amount: Uint128::try_from(145965720506419u64)?,
                total_supply: Uint128::try_from(13750071736083280u64)?,
            },
        );

        Ok(())
    }

    #[tokio::test]
    async fn v3_pools_loopmarkets() -> Result<()> {
        let factory = Addr::unchecked(dex::loopmarkets::ADDRESS);
        let height = "6751891".parse::<block::Height>()?;
        let response = ResponseTemplate::new(200).set_body_bytes(
            include_bytes!("../../fixtures/mekapi/v3-pools_loopmarkets.json").to_vec(),
        );

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("v3/pools"))
            .and(query_param("factory", &factory.to_string()))
            .and(query_param("height", &height.to_string()))
            .respond_with(response)
            .mount(&server)
            .await;

        let http = Http::new(server.uri())?;
        let pools = http.v3_pools(factory, height).await?;

        assert_eq!(pools.len(), 43);

        let star_ust_addr = Addr::unchecked("terra1lgazu0ltsxm3ayellqa2mhnhlvgx3hevkqeqy2");
        let star_ust_pool = pools
            .iter()
            .find(|pool| pool.contract_address == star_ust_addr)
            .unwrap();

        assert_eq!(star_ust_pool.contract_address, star_ust_addr);
        assert_eq!(
            star_ust_pool.liquidity_token,
            Addr::unchecked("terra16j3v7gkczclh46p6uwymrtufnq5ca4psdl04sj")
        );
        assert_eq!(star_ust_pool.total_share, Uint128::try_from(3679236962u64)?);
        assert_eq!(
            star_ust_pool.assets[0],
            v3::Asset::Cw20 {
                contract_addr: Addr::unchecked("terra13xujxcrc9dqft4p9a8ls0w3j0xnzm6y2uvve8n"),
                amount: Uint128::try_from(3489691134u64)?,
                decimals: 6,
                name: "StarTerra Token".to_string(),
                symbol: "STT".to_string(),
                total_supply: Uint128::try_from(75866627572859u64)?,
            },
        );
        assert_eq!(
            star_ust_pool.assets[1],
            v3::Asset::Native {
                amount: Uint128::try_from(4086746967u64)?,
                denom: "uusd".parse::<Denom>().unwrap(),
                total_supply: Uint128::try_from(13764802755811511u64)?,
            },
        );

        Ok(())
    }

    #[tokio::test]
    async fn v3_pools_terraswap() -> Result<()> {
        let factory = Addr::unchecked(dex::terraswap::ADDRESS);
        let height = "6759772".parse::<block::Height>()?;
        let response = ResponseTemplate::new(200).set_body_bytes(
            include_bytes!("../../fixtures/mekapi/v3-pools_terraswap.json").to_vec(),
        );

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("v3/pools"))
            .and(query_param("factory", &factory.to_string()))
            .and(query_param("height", &height.to_string()))
            .respond_with(response)
            .mount(&server)
            .await;

        let http = Http::new(server.uri())?;
        let pools = http.v3_pools(factory, height).await?;

        assert_eq!(pools.len(), 978);

        let luna_ust_addr = Addr::unchecked("terra1tndcaqxkpc5ce9qee5ggqf430mr2z3pefe5wj6");
        let luna_ust_pool = pools
            .iter()
            .find(|pool| pool.contract_address == luna_ust_addr)
            .unwrap();

        assert_eq!(luna_ust_pool.contract_address, luna_ust_addr);
        assert_eq!(
            luna_ust_pool.liquidity_token,
            Addr::unchecked("terra17dkr9rnmtmu7x4azrpupukvur2crnptyfvsrvr")
        );
        assert_eq!(
            luna_ust_pool.total_share,
            Uint128::try_from(1140364070981u64)?
        );
        assert_eq!(
            luna_ust_pool.assets[0],
            v3::Asset::Native {
                denom: "uusd".parse::<Denom>().unwrap(),
                amount: Uint128::try_from(24750209424590u64)?,
                total_supply: Uint128::try_from(13846972254844578u64)?,
            },
        );
        assert_eq!(
            luna_ust_pool.assets[1],
            v3::Asset::Native {
                denom: "uluna".parse::<Denom>().unwrap(),
                amount: Uint128::try_from(312056511288u64)?,
                total_supply: Uint128::try_from(779103799509128u64)?,
            },
        );

        Ok(())
    }
}
