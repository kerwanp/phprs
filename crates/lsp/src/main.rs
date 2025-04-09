use anyhow::Result;
use phprs_lsp::run;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    run().await
}
