use service::HelloClient;
use tarpc::{client, context, tokio_serde::formats::Json};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut transport = tarpc::serde_transport::tcp::connect("[::1]:50051", Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let client = HelloClient::new(client::Config::default(), transport.await?).spawn();

    let response = client
        .hello(context::current(), "tarpc".to_string())
        .await?;

    println!("{:?}", response);

    Ok(())
}
