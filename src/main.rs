use anyhow::{Context, Result};
use clap::Parser;
use std::env;

mod date;
mod doc;
mod input_service;
mod rask_api;
mod rask_command;

use rask_command::*;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: RaskCommand,
}

fn main() -> Result<()> {
    // .envファイルの読み込み
    dotenv::dotenv().context("envファイルの読み込みに失敗しました")?;

    // コマンドライン引数の解析
    let cli = Cli::parse();
    let command_name = format!("{:?}", cli.command);

    // RaskApiClient の作成
    let token =
        env::var("RASK_API_TOKEN").context("環境変数 RASK_API_TOKEN の読み込みに失敗しました")?;
    let url = env::var("API_BASE_URL").context("環境変数 API_BASE_URL の読み込みに失敗しました")?;
    let api_client = rask_api::RaskApiClient::new(token, url);

    // コマンドの実行
    cli.command
        .execute(api_client)
        .with_context(|| format!("コマンド'{:?}'の実行に失敗しました", command_name))?;

    Ok(())
}
