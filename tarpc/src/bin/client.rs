use service::HelloClient;
use tarpc::{client, context, tokio_serde::formats::Json};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut transport = tarpc::serde_transport::tcp::connect("[::1]:50051", Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let client = HelloClient::new(client::Config::default(), transport.await?).spawn();

    match client
        .hello(context::current(), format!("{}", "Jiannei"))
        .await
    {
        Ok(hello) => println!("{:?}", hello),
        Err(e) => panic!("{:?}", anyhow::Error::from(e)),
    }

    Ok(())
}
