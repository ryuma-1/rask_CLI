use anyhow::{Context, Result};
use clap::Parser;

mod date;
mod doc;
mod input_service;
mod minute;
mod rask_api;
mod rask_command;
mod task;

use rask_command::*;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: RaskCommand,
}

fn main() -> Result<()> {
    dotenv::dotenv().context("envファイルの読み込みに失敗しました")?;
    let cli = Cli::parse();
    let command_name = format!("{:?}", cli.command);
    cli.command
        .execute()
        .with_context(|| format!("コマンド'{:?}'の実行に失敗しました", command_name))?;
    Ok(())
}
