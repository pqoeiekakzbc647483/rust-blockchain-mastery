//! 区块链P2P网络节点 - 原创极简实现
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub struct P2PNode {
    port: u16,
    peers: Vec<String>,
}

impl P2PNode {
    pub fn new(port: u16) -> Self {
        Self { port, peers: Vec::new() }
    }

    pub fn add_peer(&mut self, peer: String) {
        self.peers.push(peer);
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        println!("P2P Node running on port {}", self.port);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            thread::spawn(move || {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                println!("Received: {}", String::from_utf8_lossy(&buffer));
            });
        }
    }

    pub fn broadcast(&self, msg: &str) {
        for peer in &self.peers {
            let mut stream = TcpStream::connect(peer).unwrap();
            stream.write_all(msg.as_bytes()).unwrap();
        }
    }
}

fn main() {
    let node = P2PNode::new(8080);
    node.start();
}
