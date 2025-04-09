use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct LspCommand {}

pub async fn run(_cmd: LspCommand) -> Result<()> {
    phprs_lsp::run().await
}
