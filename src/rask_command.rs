use std::env;
use clap::{Subcommand};
use chrono::NaiveDate;

use crate::doc;
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
    SearchDocByDate {
        date_str: String,
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

            RaskCommand::SearchDocByDate { date_str: ds } => {
                // ここはAPI側で日付検索のエンドポイントがある前提で実装
                let date: NaiveDate = ds.parse().unwrap();
                let res = api.get_all_docs()?;
                let doc_res : Vec<DocRes> = serde_json::from_str(&res.text()?)?;

                // 指定された日が開始日であるドキュメントを取得
                // let on_date_docs: Vec<DocRes> = doc_res.into_iter().filter(|doc| {
                //     doc.start_at().map_or(false, |start| start.date_naive() == date)
                // }).collect();

                // if (on_date_docs.is_empty()) {
                //     println!("指定された日付 {} に一致するドキュメントは見つかりませんでした。", date);
                //     return Ok(());
                // }

                // そのドキュメントの種類を確認
                // let doc_type = on_date_docs[0].doc_type();

                

                // date に一致するドキュメントをフィルタリング
                let filtered_docs: Vec<DocRes> = doc_res.into_iter().filter(|doc| {
                    doc.start_at().map_or(false, |start| start.date_naive() == date)
                }).collect();

                // ドキュメント数が0件の場合のメッセージ
                if filtered_docs.is_empty() {
                    println!("指定された日付 {} に一致するドキュメントは見つかりませんでした。", date);
                    return Ok(());
                }

                // フィルタリングされたurlを表示
                for doc in filtered_docs {
                    println!("url: {}", doc.url().value());
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
