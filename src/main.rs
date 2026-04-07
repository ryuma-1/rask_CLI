use std::env;
use std::io;
use clap::{Parser, Subcommand};

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
        
        let base_url = "http://localhost:3000";

        // 2. match の中身を整理
        match self {
            Commands::GetAllTasks {} => {
                let res = client
                    .get(&format!("{}/tasks.json?api_token={}", base_url, token))
                    .send()?; // 送信し忘れを修正

                print_response(res)?;
                Ok(())
            }

            Commands::GetTask { path } => {
                let res = client
                    .get(&format!("{}/tasks/{}.json?api_token={}", base_url, path, token))
                    .send()?;

                print_response(res)?;
                Ok(())
            }

            Commands::CreateTask {} => {
                let data = serde_json::json!({
                    "task": {
                        "assigner_id": input("assigner_id:"),
                        "due_at":      input("due_at:"),
                        "task_state_id": input("task_state_id:"),
                        "content":     input("content:"),
                        "description": input("description:"),
                    }
                });

                let res = client
                    .post(&format!("{}/tasks.json?api_token={}", base_url, token))
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

