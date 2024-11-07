use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let runtime = tokio::runtime::Runtime::new()?;

    #[cfg(feature = "fs")]
    runtime.block_on(test_fs())?;
    #[cfg(feature = "redis")]
    runtime.block_on(test_redis())?;

    println!("hello,world");

    Ok(())
}

// https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html
#[cfg(feature = "fs")]
async fn test_fs() -> Result<()> {
    use opendal::{layers::LoggingLayer, services::Fs, Operator};
    use std::path::PathBuf;

    let base_path = std::env::var("CARGO_MANIFEST_DIR").map(PathBuf::from)?;

    // Create fs backend builder.
    let builder = Fs::default()
        // Set the root for fs, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        .root(&base_path.join("storage").as_path().to_string_lossy());

    // `Accessor` provides the low level APIs, we will use `Operator` normally.
    let op: Operator = Operator::new(builder)?
        .layer(LoggingLayer::default())
        .finish();

    op.write("hello.log", "Hello OpenDAL!").await?;

    let _content = op.read("hello.log").await?.current();

    let _stat = op.stat("hello.log").await?;

    op.copy("hello.log", "hello2.log").await?;

    Ok(())
}

#[cfg(feature = "redis")]
async fn test_redis() -> Result<()> {
    use opendal::{layers::LoggingLayer, services::Redis, Operator};

    let builder = Redis::default().root("/opendal:");

    // this will build an Operator accessing Redis which runs on
    // tcp://localhost:6379
    let op: Operator = Operator::new(builder)?
        .layer(LoggingLayer::default())
        .finish();

    op.write("a", "b").await?;
    op.read("a").await?;

    Ok(())
}
