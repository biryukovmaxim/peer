use crate::peer_provider::PeerProvider;
use config::Config;
use dashmap::DashMap;
use std::error::Error;
use std::sync::Arc;

#[path = "../config.rs"]
mod config;
#[path = "../delivery/mod.rs"]
mod delivery;
#[path = "../errors.rs"]
mod errors;
#[path = "../use_case/peer_provider.rs"]
mod peer_provider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let my_config = Config::read_config()?;
    let peer_storage = DashMap::with_capacity(2);
    peer_storage.insert(my_config.my_address.clone(), false);
    if let Some(remote_peer) = my_config.known_peer_address.clone() {
        peer_storage.insert(remote_peer.clone(), false);
    }
    let peer_provider = PeerProvider::new(
        peer_storage,
        my_config.my_address.clone(),
        my_config.broadcast_size,
    );
    let peer_provider = Arc::new(peer_provider);

    if let Some(known_peer) = my_config.known_peer_address {
        let peer_provider = peer_provider.clone();
        tokio::spawn(async move {
           peer_provider.connect_to_known(known_peer).await;
       });
    }
    let server =
        delivery::grpc_server::Server::new(my_config.my_address, peer_provider, my_config.period);

    server.run().await?;
    Ok(())
}
