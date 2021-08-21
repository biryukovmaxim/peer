use std::collections::HashSet;
use std::net::TcpListener;
use std::io::{Read, Write};
use std::sync::{Mutex, Arc};

pub struct Server {
    socket_address: String,
    peers: Arc<Mutex<HashSet<String>>>
}

impl Server {
    pub fn new(socket_address: String, peers: Arc<Mutex<HashSet<String>>>) -> Self {
        Server {
            socket_address,
            peers
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let connection_listener = TcpListener::bind(&self.socket_address)?;
        println!("Running on {}", self.socket_address);
        connection_listener.accept();
        loop {
            if let Ok((mut stream, info)) = connection_listener.accept() {
                println!("{:?}", info);
                println!("Connection established");
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                stream.write(&mut buffer).unwrap();
            }
        }
    }
}

