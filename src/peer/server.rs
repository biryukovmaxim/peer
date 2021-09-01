use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use tokio::sync::broadcast::Sender;
use tokio_stream::wrappers::ReceiverStream;
use std::time::SystemTime;
use tokio_stream::StreamExt;
use tokio::sync::mpsc;
use tonic::{Status, Streaming, Request, Response};

use crate::peer::api::peer_server::Peer;
use crate::peer::api::*;

#[derive(Debug, Clone)]
pub struct Server {
    peers: Arc<Mutex<HashSet<String>>>,
   pub address: String,
    sender: Arc<Sender<String>>,
}

impl Server {
    pub(crate) fn new(peers: Arc<Mutex<HashSet<String>>>, address: String, sender: Arc<Sender<String>>) -> Self {
        Server {
            peers,
            address,
            sender,
        }
    }
}

#[tonic::async_trait]
impl Peer for Server {
    async fn greeting(
        &self,
        request : Request<PeerInfo>,
    ) -> Result<Response<PeerInfoList>, Status> {
        let remote_address = request.into_inner().address;
        match self.peers.lock() {
            Err(err) => Err(Status::internal(err.to_string())),
            Ok(mut peers) => {
                peers.insert(remote_address.clone());
                Ok(Response::new(PeerInfoList {
                    infos: peers
                        .iter()
                        .filter(|peer| **peer != remote_address)
                        .map(|peer| PeerInfo {
                            address: peer.to_string(),
                        })
                        .collect::<Vec<PeerInfo>>(),
                }))
            }
        }
    }

    type ChatStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn chat(
        &self,
        request: Request<Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ChatStream>, Status> {

        let mut inc_stream:Streaming<ChatMessage> = request.into_inner();
        while let Some(chat_message) = inc_stream.next().await {
            let chat_message:ChatMessage = chat_message?;
            // todo use logger
            println!("{:?} - Received message [{}] from \"{}\"", SystemTime::now(), chat_message.content, chat_message.from)
        }

        let (tx, rx) = mpsc::channel(4);
        let mut rx2 = self.sender.subscribe();
        let address = self.address.clone();

        tokio::spawn(async move {
            loop {
                let message_value = rx2.recv().await.unwrap();

                let send_res = tx.send(Ok(ChatMessage {
                    from: address.clone(),
                    content: message_value.clone(),
                })).await;
                match send_res {
                    // todo use logger
                    Ok(()) => println!("{:?} - Sending message [{}] to {}", SystemTime::now(), message_value, "some1"),
                    Err(err) => {
                        // todo use logger
                        println!("{}", err);
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}