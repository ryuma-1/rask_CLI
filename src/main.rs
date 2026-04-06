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
        match self {
            Commands::GetAllTasks {} => {
                let token = env::var("RASK_API_TOKEN")?;
                println!("Token: {}", token);
                let res = reqwest::blocking::get(&format!("http://localhost:3000/tasks.json?api_token={}", token))?;
                println!("Status: {}", res.status());
                let body = res.text()?;
                println!("{}", body);
                Ok(())
            }

            Commands::GetTask { path } => {
                let token = env::var("RASK_API_TOKEN")?;
                let res = reqwest::blocking::get(
                    &format!("http://localhost:3000/tasks/{}.json?api_token={}", path, token)
                )?;
                println!("Status: {}", res.status());
                let body = res.text()?;
                println!("{}", body);
                Ok(())
            }

            Commands::CreateTask {} => {
                let token = env::var("RASK_API_TOKEN")?;

                let assigner_id = input("assigner_id:");
                let due_at = input("due_at:");
                let task_state_id = input("task_state_id:");
                let content = input("content:");
                let description = input("description:");

                let client = reqwest::blocking::Client::new();

                let res = client.post(
                    &format!("http://localhost:3000/tasks.json?api_token={}", token)
                )
                .json(&serde_json::json!({
                    "task": {
                        "assigner_id": assigner_id,
                        "due_at": due_at,
                        "task_state_id": task_state_id,
                        "content": content,
                        "description": description,
                    }
                }))
                .send()?;

                println!("Status: {}", res.status());
                let body = res.text()?;
                println!("{}", body);
                Ok(())
            }
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

