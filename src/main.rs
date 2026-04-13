use clap::{Parser};

mod rask_api;
mod date;
mod task;
mod input_service;
mod rask_command;
mod doc;
mod minute;

use rask_command::*;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: RaskCommand,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // .envの読み込み
    let cli = Cli::parse();
    cli.command.execute()?;
    Ok(())
}

