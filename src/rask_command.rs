use crate::doc::*;
use crate::print_service;
use crate::rask_api::*;
use crate::task::*;

use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum RaskCommand {
    GetAllTasks {
        #[arg(long, default_value_t = false)]
        is_json: bool,
    },
    GetTask {
        id: i32,

        #[arg(long, default_value_t = false)]
        is_json: bool,
    },
    GetAllDocs {
        #[arg(long, default_value_t = false)]
        is_json: bool,
    },
    GetDoc {
        id: i32,

        #[arg(long, default_value_t = false)]
        is_json: bool,
    },
    SearchDoc {
        #[arg(long)]
        id: Option<u32>,

        #[arg(long)]
        content: Vec<String>,

        #[arg(long)]
        creator_id: Option<u32>,

        #[arg(long)]
        creator_name: Vec<String>,

        #[arg(long)]
        description: Vec<String>,

        #[arg(long)]
        created_at: Option<DateTime<Utc>>,

        #[arg(long)]
        updated_at: Option<DateTime<Utc>>,

        #[arg(long)]
        project_id: Option<u32>,

        #[arg(long)]
        project_name: Vec<String>,

        #[arg(long)]
        start_at: Option<DateTime<Utc>>,

        #[arg(long)]
        end_at: Option<DateTime<Utc>>,

        #[arg(long)]
        term_day: Option<u32>,

        #[arg(long, default_value_t = false)]
        is_json: bool,
    },
}

pub trait Executable {
    fn execute(self, rask_api: RaskApiClient) -> Result<()>;
}

impl Executable for RaskCommand {
    fn execute(self, rask_api: RaskApiClient) -> Result<()> {
        let print_service = print_service::PrintService::new();

        match self {
            RaskCommand::GetAllTasks { is_json } => {
                let res = rask_api.get_all_tasks()?;
                let task_res: Vec<TaskRes> = serde_json::from_str(&res.text()?)?;
                let task_list = TaskResList::new(task_res);

                if task_list.tasks().is_empty() {
                    eprintln!("No tasks found.");
                    return Ok(());
                }

                if is_json {
                    print_service.print_list_json(&task_list)?;
                } else {
                    print_service.print_list(&task_list)?;
                }

                Ok(())
            }

            RaskCommand::GetTask { id, is_json } => {
                let res = rask_api.get_task(id)?;
                let task_res: TaskRes = serde_json::from_str(&res.text()?)?;

                if is_json {
                    print_service.print_json(&task_res)?;
                } else {
                    print_service.print(&task_res)?;
                }

                Ok(())
            }

            RaskCommand::GetAllDocs { is_json } => {
                let res = rask_api.get_all_docs()?;
                let doc_res: Vec<DocRes> = serde_json::from_str(&res.text()?)?;
                let doc_list = DocResList::new(doc_res);

                if doc_list.docs().is_empty() {
                    eprintln!("No documents found.");
                    return Ok(());
                }

                if is_json {
                    print_service.print_list_json(&doc_list)?;
                } else {
                    print_service.print_list(&doc_list)?;
                }

                Ok(())
            }

            RaskCommand::GetDoc { id, is_json } => {
                let res = rask_api.get_doc(id)?;
                let doc_res: DocRes = serde_json::from_str(&res.text()?)?;

                if is_json {
                    print_service.print_json(&doc_res)?;
                } else {
                    print_service.print(&doc_res)?;
                }

                Ok(())
            }

            RaskCommand::SearchDoc {
                id,
                content,
                creator_id,
                creator_name,
                description,
                created_at,
                updated_at,
                project_id,
                project_name,
                start_at,
                end_at,
                term_day,
                is_json,
            } => {
                // 1. すべてのドキュメントを取得し DocResList に変換
                let res = rask_api.get_all_docs()?;
                let doc_res: Vec<DocRes> = serde_json::from_str(&res.text()?)?;
                let doc_list = DocResList::new(doc_res);

                // 2. フィルタリング
                let result = filter_docs(
                    doc_list,
                    id,
                    content,
                    creator_id,
                    creator_name,
                    description,
                    created_at,
                    updated_at,
                    project_id,
                    project_name,
                    start_at,
                    end_at,
                    term_day,
                );

                // 3. 結果が空の場合は終了
                if result.docs().is_empty() {
                    eprintln!("No documents found matching the criteria.");
                    return Ok(());
                }

                // 4. 出力
                if is_json {
                    print_service.print_list_json(&result)?;
                } else {
                    print_service.print_list(&result)?;
                }

                Ok(())
            }
        }
    }
}

fn filter_docs(
    doc_list: DocResList,
    id: Option<u32>,
    content: Vec<String>,
    creator_id: Option<u32>,
    creator_name: Vec<String>,
    description: Vec<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    project_id: Option<u32>,
    project_name: Vec<String>,
    start_at: Option<DateTime<Utc>>,
    end_at: Option<DateTime<Utc>>,
    term_day: Option<u32>,
) -> DocResList {
    match id {
        Some(id) => doc_list.filter_by_id(id),
        None => {
            let after_text = doc_list
                .filter_by_content(&content)
                .filter_by_creator(creator_id, &creator_name)
                .filter_by_description(&description)
                .filter_by_project(project_id, &project_name);

            match term_day {
                Some(term) => {
                    after_text.filter_by_date_range(created_at, updated_at, start_at, end_at, term)
                }
                None => after_text.filter_by_date_exact(created_at, updated_at, start_at, end_at),
            }
        }
    }
}
