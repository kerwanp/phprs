use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    phprs_cli::run().await
}
