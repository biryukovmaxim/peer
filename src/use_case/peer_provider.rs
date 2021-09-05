use crate::delivery::api::ChatMessage;
use dashmap::DashMap;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::*;
use tokio::time::interval;
use tokio::time::Duration;
use tonic::Status;
use chrono::prelude::*;

pub struct PeerProvider {
    pub peers: DashMap<String, bool>,
    sender: Arc<broadcast::Sender<String>>,
    my_address: String,
    client: crate::delivery::client::Client,
}

impl PeerProvider {
    pub fn new(peers: DashMap<String, bool>, my_address: String, broadcast_size: usize) -> Self {
        let (sender, _) = broadcast::channel(broadcast_size);
        PeerProvider {
            peers,
            sender: Arc::new(sender),
            my_address: my_address.clone(),
            client: crate::delivery::client::Client::new(my_address),
        }
    }

    pub async fn connect_to_known(&self, known_peer: String) {
        let client = self.client.clone();
        match client.connect(known_peer.clone()).await {
            Ok(new_peers) => {
                self.peers.insert(known_peer.clone(), false);
                client_subscribe_process(
                    client.clone(),
                    known_peer.clone(),
                    self.peers.clone(),
                )
                .await;
                new_peers.into_iter().for_each(|peer| {
                    self.peers.entry(peer).or_insert(false);
                });
            }
            Err(_) => {
                self.peers.insert(known_peer.clone(), false);
            }
        }

        let peers_for_connect: Vec<String> = self
            .peers
            .iter_mut()
            .filter(|kv| {
                let key = kv.key();
                !*kv.value() && *key != self.my_address.clone() && *key != known_peer.clone()
            })
            .map(|kv| kv.key().clone())
            .collect();

        for peer in peers_for_connect {
            let client = self.client.clone();
            match client.connect(peer.clone()).await {
                Ok(_) => {
                    self.peers.insert(peer.clone(), false);
                    client_subscribe_process(
                        client.clone(),
                        peer.clone(),
                        self.peers.clone(),
                    ).await;
                }
                Err(_) => {
                    self.peers.entry(peer.clone()).or_insert(false);
                }
            }
        }
    }

    pub async fn run_gossip_producer(&self, period: u64) {
        let sender = self.sender.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(period));
            loop {
                interval.tick().await;
                if sender.receiver_count() == 0 {
                    continue;
                }
                let rand_string: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect();
                sender.send(rand_string).unwrap();
            }
        });
    }

    pub async fn add_peer(&self, new_peer_address: String) -> Vec<String> {
        self.peers.insert(new_peer_address.clone(), false);
        client_subscribe_process(
            self.client.clone(),
            new_peer_address.clone(),
            self.peers.clone(),
        )
        .await;
        self.peers.iter().map(|kv| kv.key().clone()).collect()
    }

    pub async fn subscribe(
        &self,
        remote_peer_address: String,
        tx: mpsc::Sender<Result<ChatMessage, Status>>,
    ) {
        let mut message_consumer = self.sender.subscribe();
        loop {
            let message_value = match message_consumer.recv().await {
                Ok(message) => message,
                Err(err) => {
                    if err == RecvError::Closed {
                        break;
                    } else {
                        continue;
                    }
                }
            };

            let send_res = tx
                .send(Ok(ChatMessage {
                    from: self.my_address.clone(),
                    content: message_value.clone(),
                }))
                .await;
            match send_res {
                // todo use logger
                Ok(()) => println!(
                    "{:?} - Sending message [{}] to {}",
                    Local::now(),
                    message_value,
                    &remote_peer_address
                ),
                Err(err) => {
                    // todo use logger
                    println!("{}", err);
                    break;
                }
            }
        }
    }
}

async fn client_subscribe_process(
    client: crate::delivery::client::Client,
    remote_peer: String,
    peers: DashMap<String, bool>,
) {
    tokio::spawn(async move {
        if let Some(mut stream) = client.subscibe(remote_peer.clone()).await {
            println!(
                "{} - Connected to peer [{}]",
                Local::now(),
                &remote_peer,
            );
            peers.insert(remote_peer, true);
            while let Ok(chat_message_resp) = stream.message().await {
                if let Some(message) = chat_message_resp {
                    // todo use logger
                    println!(
                        "{} - Received message [{}] from \"{}\"",
                        Local::now(),
                        &message.content,
                        &message.from
                    );
                }
            }
        }
    });
}