#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for arg in std::env::args().skip(1) {
        let mut split = arg.split(':');
        let kind = split.next().unwrap();
        let data = split.next().unwrap();
        let data = hex::decode(data)?;
        let output = match kind {
            "terraswap" => cosmwasm_std::from_slice::<terraswap::pair::PoolResponse>(&data)
                .and_then(|resp| cosmwasm_std::to_vec(&resp)),

            "astroport" => cosmwasm_std::from_slice::<astroport::pair::PoolResponse>(&data)
                .and_then(|resp| cosmwasm_std::to_vec(&resp)),

            "loopmarkets" => cosmwasm_std::from_slice::<terraswap::pair::PoolResponse>(&data)
                .and_then(|resp| cosmwasm_std::to_vec(&resp)),

            _ => panic!("{arg}: unknown"),
        }?;

        println!("{}", std::string::String::from_utf8_lossy(&output));
    }

    Ok(())
}
