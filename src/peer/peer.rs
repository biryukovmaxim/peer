use crate::peer::server::Server as PeerServerImpl;
use tonic::transport::Server;
use crate::errors::PeerError;
use crate::peer::api::peer_server::PeerServer;
use crate::peer::api::peer_client::PeerClient;
use crate::peer::api::*;

#[derive(Debug, Clone)]
pub struct Peer {
    server: PeerServerImpl,
    known_peer: Option<String>,
    my_server_addr: String,
}

impl Peer {
    pub(crate) fn new(server: PeerServerImpl, my_server_addr:String, known_peer: Option<String>) -> Self{
        Peer {
            server,
            known_peer,
            my_server_addr,
        }
    }

    pub(crate) async fn run(self) -> Result<(), PeerError>{

        if let Some(address) = self.known_peer {
            let mut client = PeerClient::connect(format!("http://{}", &address)).await?;
            let request = tonic::Request::new(PeerInfo {
                address: self.my_server_addr.clone(),
            });

            let response = client.greeting(request).await?;

            println!("RESPONSE={:?}", response);
        }

        let address = self.server.address.clone();
        Server::builder()
            .add_service(PeerServer::new(self.server))
            .serve(address.parse()?)
            .await
            .map_err(|err| err.into())
    }
}

