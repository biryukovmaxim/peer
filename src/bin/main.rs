use crate::peer::PeerService;
use config::Config;
use std::error::Error;

#[path = "../config.rs"]
mod config;
#[path = "../errors.rs"]
mod errors;
#[path = "../peer.rs"]
mod peer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::read_config()?;
    let peer = PeerService::new(config);
    peer.run().await?;
    Ok(())
}
