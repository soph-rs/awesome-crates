use anyhow::Result;
use opendal::layers::LoggingLayer;
use opendal::services::{Fs, Redis};
use opendal::Operator;
use std::path::PathBuf;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(test_fs())?;
    runtime.block_on(test_redis())?;

    println!("hello,world");

    Ok(())
}

// https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html
async fn test_fs() -> Result<()> {
    let base_path = std::env::var("CARGO_MANIFEST_DIR").map(PathBuf::from)?;

    // Create fs backend builder.
    let mut builder = Fs::default()
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

async fn test_redis() -> Result<()> {
    let mut builder = Redis::default().root("/opendal:");

    // this will build a Operator accessing Redis which runs on tcp://localhost:6379
    let op: Operator = Operator::new(builder)?
        .layer(LoggingLayer::default())
        .finish();

    op.write("a", "b").await?;
    op.read("a").await?;

    Ok(())
}
