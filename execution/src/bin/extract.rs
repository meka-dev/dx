use tendermint_rpc::{Client as _, SubscriptionClient as _, WebSocketClient};
use terrars::Msg as _;

use execution::config;

const TERRASWAP_LUNAUST_PAIR_ADDRESS: &str = "terra1tndcaqxkpc5ce9qee5ggqf430mr2z3pefe5wj6";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Set up RPC client.
    let (client, driver) = WebSocketClient::new(config::RPC_URL).await?;
    // Advance the internal machinery of the RPC client.
    let driver_handle = tokio::spawn(driver.run());

    // Fetch example block.
    let block = client.block::<u32>(4812719).await?;

    // Print metadata.
    println!("ID: {}", block.block_id);
    println!("HEIGHT: {}", block.block.header.height);
    println!("TXs: {}", block.block.data.iter().len());

    // Iterate over transactions and extract contract data.
    for raw in block.block.data.iter() {
        println!("\n\n");
        let tx = terrars::Tx::from_bytes(raw.as_bytes())?;

        println!("{}", tx.body.messages.len());

        for any in tx.body.messages {
            let url = any.type_url.as_str();
            println!("\t{}", url);

            if any.type_url == "/terra.wasm.v1beta1.MsgExecuteContract" {
                let msg = terrars::MsgExecuteContract::from_any(&any)?;

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

    client.close()?;
    driver_handle.await??;

    Ok(())
}
