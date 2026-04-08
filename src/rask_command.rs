use std::env;
use clap::{Subcommand};

use crate::task::*;
use crate::rask_api::*;
use crate::input_service::InputUtils;

#[derive(Subcommand)]
pub enum RaskCommand {
    GetAllTasks {
    },
    GetTask {
        id: i32,
    },
    CreateTask {
    },
}

pub trait Executable {
    fn execute(self) -> Result<(), Box<dyn std::error::Error>>;
}

impl Executable for RaskCommand {
    fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        // 1. クライアントとトークンを共通で準備
        let token = env::var("RASK_API_TOKEN")
            .map_err(|_| "環境変数 RASK_API_TOKEN が設定されていません。")?;
        let url = env::var("API_BASE_URL")
            .map_err(|_| "環境変数 API_BASE_URL が設定されていません。")?;

        let api = RaskApiClient::new(token, url);

            // 2. match の中身を整理
        match self {
            RaskCommand::GetAllTasks {} => {

                let res = api.get_all_tasks()?;

                print_response(res)?;
                Ok(())
            }

            RaskCommand::GetTask { id } => {
                let res = api.get_task(id)?;
                print_response(res)?;
                Ok(())
            }

            RaskCommand::CreateTask {} => {

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

                let res = api.create_task(data)?;

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
