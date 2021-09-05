use crate::delivery::api::peer_client::PeerClient;
use crate::delivery::api::{ChatMessage, PeerInfo};
use tonic::{Request, Streaming};

#[derive(Debug, Clone)]
pub struct Client {
    my_address: String,
}

impl Client {
    pub async fn subscibe(&self, remote: String) -> Option<Streaming<ChatMessage>> {
        if let Ok(mut client) = PeerClient::connect(format!("http://{}", &remote)).await {
            if let Ok(stream_resp) = client
                .subscribe(Request::new(PeerInfo {
                    address: self.my_address.clone(),
                }))
                .await
            {
                return Some(stream_resp.into_inner());
            }
        }
        None
    }

    pub async fn connect(&self, remote: String) -> Result<Vec<String>, ()> {
        if let Ok(mut client) = PeerClient::connect(format!("http://{}", &remote)).await {
            if let Ok(stream_resp) = client
                .add_peer(Request::new(PeerInfo {
                    address: self.my_address.clone(),
                }))
                .await
            {
                return Ok(stream_resp
                    .into_inner()
                    .infos
                    .into_iter()
                    .map(|peer| peer.address)
                    .collect());
            }
        }
        Err(())
    }
}

impl Client {
    pub fn new(my_address: String) -> Self {
        Client { my_address }
    }
}
