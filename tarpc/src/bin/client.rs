use service::HelloClient;
use std::net::{IpAddr, Ipv6Addr};
use tarpc::{client, context, tokio_serde::formats::Json};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_addr = (IpAddr::V6(Ipv6Addr::LOCALHOST), 50051);
    let mut transport = tarpc::serde_transport::tcp::connect(&server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let client = HelloClient::new(client::Config::default(), transport.await?).spawn();

    let response = client
        .hello(context::current(), "tarpc".to_string())
        .await?;

    println!("{:?}", response);

    Ok(())
}
