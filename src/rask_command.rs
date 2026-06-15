use crate::doc::*;
use crate::rask_api::*;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum RaskCommand {
    GetAllTasks {},
    GetTask {
        id: i32,
    },
    GetAllDocs {},
    GetDoc {
        id: i32,
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
        is_visual: bool,
    },
}

pub trait Executable {
    fn execute(self, rask_api: RaskApiClient) -> Result<()>;
}

impl Executable for RaskCommand {
    fn execute(self, rask_api: RaskApiClient) -> Result<()> {
        match self {
            RaskCommand::GetAllTasks {} => {
                let res = rask_api.get_all_tasks()?;
                print_response(res)?;
                Ok(())
            }

            RaskCommand::GetTask { id } => {
                let res = rask_api.get_task(id)?;
                print_response(res)?;
                Ok(())
            }

            RaskCommand::GetAllDocs {} => {
                let res = rask_api.get_all_docs()?;
                print_response(res)?;
                // print_response(res)?;
                Ok(())
            }

            RaskCommand::GetDoc { id } => {
                let res = rask_api.get_doc(id)?;
                print_response(res)?;
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
                is_visual,
            } => {
                // 1. すべてのドキュメントを取得し DocResList に変換
                let res = rask_api.get_all_docs()?;
                let doc_res: Vec<DocRes> = serde_json::from_str(&res.text()?)?;
                let doc_list = DocResList::new(doc_res);

                // 2. 各粒度のフィルタを順に適用
                let result = if is_id_match(id) {
                    doc_list.filter_by_id(id)
                } else {
                    let after_text = doc_list
                        .filter_by_content(&content)
                        .filter_by_creator(creator_id, &creator_name)
                        .filter_by_description(&description)
                        .filter_by_project(project_id, &project_name);

                    match term_day {
                        Some(term) => after_text
                            .filter_by_date_range(created_at, updated_at, start_at, end_at, term),
                        None => after_text
                            .filter_by_date_exact(created_at, updated_at, start_at, end_at),
                    }
                };

                // 3. 結果が空の場合は早期リターン
                if result.docs().is_empty() {
                    eprintln!("No documents found matching the criteria.");
                    return Ok(());
                }

                // 4. 出力
                if !is_visual {
                    let json_array = serde_json::to_string(result.docs())?;
                    println!("{}", json_array);
                } else {
                    println!(
                        "Found {} documents matching the criteria:",
                        result.docs().len()
                    );
                    for doc in result.docs() {
                        println!("ID: {}", doc.id().value());
                        println!("Content: {}", doc.content().value());
                        println!(
                            "Creator: {} (ID: {})",
                            doc.creator().name().value(),
                            doc.creator().id().value()
                        );
                        println!(
                            "Start At: {}",
                            doc.start_at()
                                .map_or("None".to_string(), |sa| sa.to_string())
                        );
                        println!(
                            "End At: {}",
                            doc.end_at().map_or("None".to_string(), |ea| ea.to_string())
                        );
                    }
                }

                Ok(())
            }
        }
    }
}

fn print_response(res: reqwest::blocking::Response) -> anyhow::Result<()> {
    println!("Status: {}", res.status());
    let body = res.text()?;
    println!("{}", body);
    Ok(())
}

fn is_id_match(id: Option<u32>) -> bool {
    id.is_none()
}
