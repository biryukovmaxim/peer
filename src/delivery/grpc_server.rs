use crate::delivery::api::peer_server::{Peer, PeerServer};
use crate::delivery::api::{ChatMessage, PeerInfo, PeerInfoList};
use crate::errors::PeerError;
use crate::peer_provider::PeerProvider;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub struct Server {
    local_address: String,
    peer_provider: Arc<PeerProvider>,
    period: u64,
}

impl Server {
    pub async fn run(self) -> Result<(), PeerError> {
        self.peer_provider.run_gossip_producer(self.period).await;
        let address = self.local_address.clone();
        let cloned_address = address.clone();
        tokio::spawn(async move {
           println!("My Peer address is \"{}\"", cloned_address)
        });
        tonic::transport::Server::builder()
            .add_service(PeerServer::new(self))
            .serve(address.parse()?)
            .await
            .map_err(|err| PeerError::CannotStartServer(err.to_string()))
    }
    pub fn new(local_address: String, peer_provider: Arc<PeerProvider>, period: u64) -> Self {
        Server {
            local_address,
            peer_provider,
            period,
        }
    }
}

#[async_trait]
impl Peer for Server {
    async fn add_peer(&self, request: Request<PeerInfo>) -> Result<Response<PeerInfoList>, Status> {
        let res = self
            .peer_provider
            .add_peer(request.into_inner().address)
            .await;
        Ok(Response::new(PeerInfoList {
            infos: res
                .into_iter()
                .map(|peer_address| PeerInfo {
                    address: peer_address,
                })
                .collect(),
        }))
    }

    type SubscribeStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn subscribe(
        &self,
        request: Request<PeerInfo>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let peer_provider = self.peer_provider.clone();
        tokio::spawn(async move {
            peer_provider
                .subscribe(request.into_inner().address, tx)
                .await;
        });
        Ok(Response::new(ReceiverStream::new(rx.into())))
    }
}
