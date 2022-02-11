#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    for arg in std::env::args().skip(1) {
        match hex::decode(arg) {
            Err(e) => return Err(e.into()),
            Ok(b) => {
                match cosmwasm_std::from_slice::<terraswap::pair::PoolResponse>(b.as_slice()) {
                    Err(e) => return Err(e.into()),
                    Ok(resp) => match cosmwasm_std::to_vec(&resp) {
                        Err(e) => return Err(e.into()),
                        Ok(output) => println!("{}", std::string::String::from_utf8_lossy(&output)),
                    },
                }
            },
        }
    }

    Ok(())
}
