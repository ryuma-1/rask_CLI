use std::env;
use std::io;
use clap::{Parser, Subcommand};

mod date;
mod task;
use task::*;

const API_BASE_URL: &str = "http://localhost:3000";

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
        
            // 2. match の中身を整理
        match self {
            Commands::GetAllTasks {} => {
                let res = client
                    .get(&format!("{}/tasks.json?api_token={}", API_BASE_URL, token))
                    .send()?; // 送信し忘れを修正

                print_response(res)?;
                Ok(())
            }

            Commands::GetTask { path } => {
                let res = client
                    .get(&format!("{}/tasks/{}.json?api_token={}", API_BASE_URL, path, token))
                    .send()?;

                print_response(res)?;
                Ok(())
            }

            Commands::CreateTask {} => {

                let data = serde_json::json!({
                    "task": {
                        "assigner_id": input_continue::<AssignerId>("assigner_id:").value(),
                        "content":     input_continue::<Content>("content:").value(),
                        "due_at":      input_continue::<DueAt>("due_at:").to_string(),
                        "description": input_continue::<Description>("description:").value(),
                        "project_id":  input_continue::<ProjectId>("project_id:").value(),
                        "task_state_id": input_continue::<TaskStateId>("task_state_id:").value(),
                    }
                });

                let res = client
                    .post(&format!("{}/tasks.json?api_token={}", API_BASE_URL, token))
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

fn input_continue<T: FromString>(title: &str) -> T {
    let input_text = input(title); // input関数がどこかで定義されている前提

    match T::new(input_text) {
        Ok(object) => {
            object
        },
        Err(e) => {
            // 失敗：エラーを表示して再試行（結果を必ずreturnする）
            eprintln!("入力エラー: {}", e);
            input_continue::<T>(title)
        }
    }
}

fn input(string: &str) -> String {
    let mut input = String::new();
    println!("{}", string);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let cli = Cli::parse();
    cli.command.execute()?;
    Ok(())
}

