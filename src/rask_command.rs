use std::env;
use clap::{Subcommand};
use chrono::NaiveDate;
use regex::Regex;
use anyhow::{Context, Result, anyhow};

use crate::doc;
use crate::minute;
use crate::minute::*;
use crate::task::*;
use crate::doc::*;
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
    GetAllDocs {
    },
    GetDoc {
        id: i32,
    },
    CreateDoc {
    },
    SearchTodayDoc {
        #[arg(value_enum)]
        m_type : MinuteType,
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
                let assigner_id = InputUtils::execute::<AssignerId>("assigner_id:");
                let content = InputUtils::execute::<Content>("content:");
                let due_at = InputUtils::execute::<DueAt>("due_at:");
                let description = InputUtils::execute::<Description>("description:");
                let project_id = InputUtils::execute::<ProjectId>("project_id:");
                let task_state_id = InputUtils::execute::<TaskStateId>("task_state_id:");

                let task = Task::new (
                    assigner_id,
                    content,
                    due_at,
                    description,
                    project_id,
                    task_state_id,
                );

                let task_req = TaskReq::new(task);
                let json = serde_json::to_value(&task_req)?;

                let res = api.create_task(json)?;

                print_response(res)?;
                Ok(())
            }


            RaskCommand::GetAllDocs {  } => {
                let res = api.get_all_docs()?;
                print_response(res)?;
            
                // print_response(res)?;
                Ok(())
            }

            RaskCommand::GetDoc { id } => {
                let res = api.get_doc(id)?;
                print_response(res)?;
                Ok(())
             }

            RaskCommand::CreateDoc {}=> {
                let content = InputUtils::execute::<Content>("content:");
                let description = InputUtils::execute::<Description>("description:");
                let project_id = InputUtils::execute::<ProjectId>("project_id:");
                let start_at = InputUtils::execute::<StartAt>("start_at:");
                let end_at = InputUtils::execute::<EndAt>("end_at:");
                let location = InputUtils::execute::<Location>("location:");

                let doc_req = DocReq::new (
                    content,
                    description,
                    project_id,
                    start_at,
                    end_at,
                    location,
                );

                let json = serde_json::to_value(&doc_req)?;

                let res = api.create_doc(json)?;

                print_response(res)?;
                Ok(())
            }

            RaskCommand::SearchTodayDoc { m_type } => {
                // ここはAPI側で日付検索のエンドポイントがある前提で実装
                // let date: NaiveDate = ds.parse().unwrap();
                let res = api.get_all_docs()?;
                let doc_res : Vec<DocRes> = serde_json::from_str(&res.text()?)?;

                // タイトルにGNやNewが含まれているドキュメントをフィルタリング
                let filtered_type_docs: Vec<DocRes> = doc_res.into_iter().filter(|doc| {
                   doc.content().to_type() == m_type
                }).collect();

                // 戻り値を Result<Vec<Minute>> にする
                let minutes: Result<Vec<Minute>> = filtered_type_docs.into_iter().map(|doc| {
                    let content = doc.content().value();
                    let num = MinuteNum::new(&content)?; // ここで ? を使ってエラーを上に投げる

                    Ok(Minute::new(m_type, num, doc.url().clone()))
                }).collect(); // Result の Vec を collect すると、自動的に Result<Vec> になる

                // 1. まず Vec を取り出して、変数にしっかり固定する
                let minutes_vec = minutes?; 

                // 2. 変数に固定された Vec に対して iter() を呼ぶ
                // これで Vec はこのスコープが終わるまで生存します
                let max_minute = minutes_vec
                    .iter()
                    .max_by_key(|minute| minute.num().value());

                match max_minute {
                    Some(minute) => {
                        println!("{}", minute.url().trim_json());
                    }
                    None => {
                        eprintln!("Error: max_minute is None"); // Ruby側で 2>&1 すれば見える
                    }
                }
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
