use anyhow::Result;
use clap::Parser;

mod date;
mod doc;
mod input_service;
mod minute;
mod rask_api;
mod rask_command;
mod task;

use rask_command::*;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: RaskCommand,
}

fn main() -> Result<()> {
    dotenv::dotenv()?;
    let cli = Cli::parse();
    cli.command.execute()?;
    Ok(())
}
