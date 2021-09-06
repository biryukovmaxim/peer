Peer is simple p2p gossiping application in Rust.
The peer has a cli interface to start it and connect itself to the other peers. Once connected, the peer should send a random gossip message to all the other peers every N seconds

#### Run peer without outgoing connects at start: `cargo run --bin peer -- --port 3001 --period 10`
#### Run peer, which will connect to the first peer: `cargo run --bin peer -- --port 3002 --connect 3001 --period 10`
#### Help: `cargo run --bin peer -- --help`
