use std::time::Duration;

use tendermint_rpc::{Client as _, SubscriptionClient, WebSocketClient};
use terra_rust_api::Terra;
use terrars::Msg as _;

mod contract;

const CHAIN_ID: &str = "columbus-5";
const LCD_URL: &str = "http://89.58.28.221:1317";
const RPC_URL: &str = "ws://89.58.28.221:26657/websocket";

const TERRASWAP_LUNAUST_PAIR_ADDRESS: &str = "terra1tndcaqxkpc5ce9qee5ggqf430mr2z3pefe5wj6";

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Node is still catching up")]
    NodeCatchingUp,
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Rpc(#[from] tendermint_rpc::Error),
    #[error(transparent)]
    Terrars(#[from] terrars::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let lcd = Terra::lcd_client_no_tx(LCD_URL, CHAIN_ID);

    let uusd = terraswap::asset::AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };
    let terraswap_pairs = contract::terraswap::liquid_pairs(&lcd, Some(uusd))
        .await
        .unwrap();

    println!("Liquid pairs: {}", terraswap_pairs.len());

    // loop {
    //     tokio::time::sleep(Duration::from_millis(500)).await;

    //     // Update prices
    //     // run strategies
    //     // broadcast txs
    // }

    return Ok(());

    let (client, driver) = WebSocketClient::new(RPC_URL).await?;
    let driver_handle = tokio::spawn(driver.run());

    let status = client.status().await?;

    if status.sync_info.catching_up {
        return Err(Error::NodeCatchingUp);
    }

    // Fetch example block.
    let block = client.block::<u32>(4812719).await?;

    // Print metadata.
    println!("ID: {}", block.block_id);
    println!("HEIGHT: {}", block.block.header.height);
    println!("TXs: {}", block.block.data.iter().len());

    // Iterate over transactions and extract contract data.
    for raw in block.block.data.iter() {
        println!("\n\n");
        let tx = terrars::Tx::from_bytes(raw.as_bytes()).unwrap();

        println!("{}", tx.body.messages.len());

        for any in tx.body.messages {
            let url = any.type_url.as_str();
            println!("\t{}", url);

            if any.type_url == "/terra.wasm.v1beta1.MsgExecuteContract" {
                let msg = terrars::MsgExecuteContract::from_any(&any).unwrap();

                println!("CONTRACT: {}", msg.contract);
                if msg.contract == TERRASWAP_LUNAUST_PAIR_ADDRESS {
                    let exe: terraswap::pair::ExecuteMsg =
                        serde_json::from_slice(&msg.execute_msg).unwrap();
                    println!("\t\t{:?}", msg);
                    println!("\t\t{:?}", exe);
                }
            }
        }
    }

    // println!("{:?}", status);

    // loop {
    //     sleep(Duration::from_millis(500)).await;

    //     let res = client.unconfirmed_txs().await?;

    //     for data in res.txs {
    //         let tx = Tx::from_bytes(&data.as_bytes()).unwrap();

    //         for any in tx.body.messages {
    //             let url = any.type_url.as_str();
    //             println!("\t\turl: {:?}", url);

    //             let msg = match url {
    //                 "/terra.wasm.v1beta1.MsgExecuteContract" => {
    //                     let mut copy = any.clone();
    //                     copy.type_url = "/cosmwasm.wasm.v1.MsgExecuteContract".to_string();

    //                     let msg = MsgExecuteContract::from_any(&copy).unwrap();

    //                     println!("\t\t\t{:?}", msg);
    //                     println!("\t\t\t{:?}", std::str::from_utf8(&msg.msg).unwrap());
    //                 }
    //                 _ => {}
    //             };
    //         }
    //     }
    // }

    // let query = Query::from(EventType::NewBlock);
    // let mut blocks = client.subscribe(query).await?;

    // while let Some(res) = blocks.next().await {
    //     let block = res.unwrap();

    //     match block.data {
    //         EventData::NewBlock {
    //             block: Some(block),
    //             result_begin_block,
    //             result_end_block,
    //         } => {
    //             println!("{} @ {}", block.header.height, block.header.time);
    //         }
    //         _ => {}
    //     }
    // }

    // let query = Query::from(EventType::Tx).and_eq("wasm.action", "swap");
    // let mut swaps = client.subscribe(query).await?;

    // while let Some(res) = swaps.next().await {
    //     let swap = res.unwrap();

    //     match swap.data {
    //         EventData::Tx { tx_result: info } => {
    //             println!(
    //                 "{}/{:?} | Gas(Wanted/Used) {}/{}",
    //                 info.height,
    //                 info.index,
    //                 info.result.gas_wanted.unwrap_or("0".into()),
    //                 info.result.gas_used.unwrap_or("0".into())
    //             );

    //             for event in info.result.events {
    //                 println!("\t{}", event.type_str);

    //                 for tag in event.attributes {
    //                     println!("\t\t{} -> {}", tag.key, tag.value);
    //                 }
    //             }
    //         }
    //         _ => {}
    //     }
    // }

    client.close()?;
    driver_handle.await?.map_err(Error::from)
}
