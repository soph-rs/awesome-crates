use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use futures::{future, prelude::*};
use service::Hello;
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Json,
};

#[derive(Clone)]
struct HelloServer(SocketAddr);

impl Hello for HelloServer {
    async fn hello(self, _: context::Context, name: String) -> String {
        format!("Hello, {name}! You are connected from {}", self.0)
    }
}

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_addr = (IpAddr::V6(Ipv6Addr::LOCALHOST), 50051); // IpAddr::from([0, 0, 0, 0])
    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
    listener.config_mut().max_frame_length(usize::MAX);

    listener
        .filter_map(|r| future::ready(r.ok())) // Ignore accept errors.
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip()) // Limit channels to 1 per IP.
        .map(|channel| {
            let server = HelloServer(channel.transport().peer_addr().unwrap());
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10) // Max 10 channels.
        .for_each(|_| async {})
        .await;
    Ok(())
}
