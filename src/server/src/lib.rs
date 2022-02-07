mod command;
mod parse;
mod reply;
mod service;

pub(crate) use service::Service;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::spawn;
use tracing::info;

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn run(self) {
        info!("listen on {}", self.addr);
        let listen = TcpListener::bind(self.addr).await.unwrap();

        loop {
            let (stream, addr) = listen.accept().await.unwrap();
            info!("client {}", addr);
            let service = Service::new(stream);
            spawn(service.run());
        }
    }
}
