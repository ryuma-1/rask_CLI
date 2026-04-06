use std::env;
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
}

trait Executable {
    fn execute(self) -> Result<(), Box<dyn std::error::Error>>;
}

impl Executable for Commands {
    fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Commands::GetAllTasks {} => {
                let token = env::var("RASK_API_TOKEN")?;
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
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    cli.command.execute()?;

    Ok(())
}

