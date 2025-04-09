mod lsp;
mod parse;

use anyhow::Result;
use clap::{Parser, Subcommand};
use lsp::LspCommand;
use parse::ParseCommand;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Lsp(LspCommand),
    Parse(ParseCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    env_logger::builder().init();

    let cli = Cli::parse();

    match cli.command {
        Command::Lsp(cmd) => lsp::run(cmd).await,
        Command::Parse(cmd) => parse::run(cmd),
    }
}
