use anyhow::Result;
use phprs_cli::run;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    env_logger::builder().init();

    run().await
}
