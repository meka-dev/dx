#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    for arg in std::env::args().skip(1) {
        let which = arg.split(":").nth(0).unwrap();
        let hex = arg.split(":").nth(1).unwrap();
        let bytes = hex::decode(hex).unwrap();
        let output = match which {
            "terraswap" => cosmwasm_std::from_slice::<terraswap::pair::PoolResponse>(&bytes)
                .and_then(|resp| cosmwasm_std::to_vec(&resp)),

            "astroport" => cosmwasm_std::from_slice::<astroport::pair::PoolResponse>(&bytes)
                .and_then(|resp| cosmwasm_std::to_vec(&resp)),

            _ => panic!("{}: unknown", arg),
        }
        .unwrap();

        println!("{}", std::string::String::from_utf8_lossy(&output));
    }

    Ok(())
}
