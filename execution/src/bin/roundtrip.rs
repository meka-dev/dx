#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    for arg in std::env::args().skip(1) {
        let mut split = arg.split(':');
        let kind = split.next().unwrap();
        let data = split.next().unwrap();
        let data = hex::decode(data).unwrap();
        let output = match kind {
            "terraswap" => cosmwasm_std::from_slice::<terraswap::pair::PoolResponse>(&data)
                .and_then(|resp| cosmwasm_std::to_vec(&resp)),

            "astroport" => cosmwasm_std::from_slice::<astroport::pair::PoolResponse>(&data)
                .and_then(|resp| cosmwasm_std::to_vec(&resp)),

            _ => panic!("{}: unknown", arg),
        }
        .unwrap();

        println!("{}", std::string::String::from_utf8_lossy(&output));
    }

    Ok(())
}
