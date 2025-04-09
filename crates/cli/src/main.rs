use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Lsp,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    env_logger::builder().init();

    let cli = Cli::parse();

    match cli.command {
        Command::Lsp => phprs_lsp::run().await,
    }
}
