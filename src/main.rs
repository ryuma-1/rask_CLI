use std::env;
use clap::{Parser, Subcommand};

mod date;
mod task;
mod input_service;
use task::*;
use input_service::InputUtils;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GetAllTasks {
    },
    GetTask {
        path: i32,
    },
    CreateTask {
    },
}

trait Executable {
    fn execute(self) -> Result<(), Box<dyn std::error::Error>>;
}

impl Executable for Commands {
    fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        // 1. クライアントとトークンを共通で準備
        let client = reqwest::blocking::Client::new();
        let token = env::var("RASK_API_TOKEN")
            .map_err(|_| "環境変数 RASK_API_TOKEN が設定されていません。")?;
        let url = env::var("API_BASE_URL")
            .map_err(|_| "環境変数 API_BASE_URL が設定されていません。")?;

            // 2. match の中身を整理
        match self {
            Commands::GetAllTasks {} => {
                let res = client
                    .get(&format!("{}/tasks.json?api_token={}", url, token))
                    .send()?; // 送信し忘れを修正

                print_response(res)?;
                Ok(())
            }

            Commands::GetTask { path } => {
                let res = client
                    .get(&format!("{}/tasks/{}.json?api_token={}", url, path, token))
                    .send()?;

                print_response(res)?;
                Ok(())
            }

            Commands::CreateTask {} => {

                let data = serde_json::json!({
                    "task": {
                        "assigner_id": InputUtils::execute::<AssignerId>("assigner_id:").value(),
                        "content":     InputUtils::execute::<Content>("content:").value(),
                        "due_at":      InputUtils::execute::<DueAt>("due_at:").to_string(),
                        "description": InputUtils::execute::<Description>("description:").value(),
                        "project_id":  InputUtils::execute::<ProjectId>("project_id:").value(),
                        "task_state_id": InputUtils::execute::<TaskStateId>("task_state_id:").value(),
                    }
                });

                let res = client
                    .post(&format!("{}/tasks.json?api_token={}", url, token))
                    .json(&data)
                    .send()?;

                print_response(res)?;
                Ok(())
            }
        }
    }
}

// レスポンス表示用の共通関数を作るとスッキリします
fn print_response(res: reqwest::blocking::Response) -> Result<(), Box<dyn std::error::Error>> {
    println!("Status: {}", res.status());
    let body = res.text()?;
    println!("{}", body);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let cli = Cli::parse();
    cli.command.execute()?;
    Ok(())
}

