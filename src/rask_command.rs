use anyhow::{anyhow, Context, Ok, Result};
use chrono::{DateTime, NaiveDate, Utc};
use clap::builder::Str;
use clap::Subcommand;
use regex::Regex;
use std::f32::consts::E;

use crate::doc;
use crate::doc::*;
use crate::input_service::InputUtils;
use crate::minute;
use crate::minute::*;
use crate::rask_api::*;
use crate::task::*;

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
                let res = rask_api.get_all_docs()?;

                let doc_res: Vec<DocRes> = serde_json::from_str(&res.text()?)?;

                // すべての条件を満たすドキュメントをフィルタリング (AND検索)
                let filtered_docs: Vec<DocRes> = doc_res
                    .into_iter()
                    .filter(|doc| {
                        (id.is_none() || id == Some(doc.id().value()))
                            && (content.is_empty()
                                || content.iter().all(|kw| doc.content().value().contains(kw)))
                            && (creator_id.is_none()
                                || creator_id == Some(doc.creator().id().value()))
                            && (creator_name.is_empty()
                                || creator_name
                                    .iter()
                                    .all(|kw| doc.creator().name().value().contains(kw)))
                            && (description.is_empty()
                                || description.iter().all(|kw| {
                                    doc.description().map_or(false, |d| d.value().contains(kw))
                                }))
                            && (project_id.is_none()
                                || project_id == Some(doc.project().map_or(0, |p| p.id().value())))
                            && (project_name.is_empty()
                                || project_name.iter().all(|kw| {
                                    doc.project()
                                        .map_or(false, |p| p.name().value().contains(kw))
                                }))
                    })
                    .collect();

                // term_day に設定した日数の前後 term_day で絞り込む
                let date_filtered_docs: Vec<DocRes>;
                if term_day.is_some() {
                    let term_duration = chrono::Duration::days(term_day.unwrap() as i64);
                    date_filtered_docs = filtered_docs
                        .into_iter()
                        .filter(|doc| {
                            (created_at.is_none()
                                || created_at.map_or(false, |ca| {
                                    ca - term_duration <= *doc.created_at()
                                        && *doc.created_at() <= ca + term_duration
                                }))
                                && (updated_at.is_none()
                                    || updated_at.map_or(false, |ua| {
                                        ua - term_duration <= *doc.updated_at()
                                            && *doc.updated_at() <= ua + term_duration
                                    }))
                                && (start_at.is_none()
                                    || start_at.map_or(false, |sa| {
                                        sa - term_duration
                                            <= doc.start_at().copied().unwrap_or_default()
                                            && doc.start_at().copied().unwrap_or_default()
                                                <= sa + term_duration
                                    }))
                                && (end_at.is_none()
                                    || end_at.map_or(false, |ea| {
                                        ea - term_duration <= doc.end_at().copied().unwrap()
                                            && doc.end_at().copied().unwrap() <= ea + term_duration
                                    }))
                        })
                        .collect();
                } else {
                    date_filtered_docs = filtered_docs
                        .into_iter()
                        .filter(|doc| {
                            (created_at.is_none()
                                || created_at.map_or(false, |ca| *doc.created_at() == ca))
                                && (updated_at.is_none()
                                    || updated_at.map_or(false, |ua| *doc.updated_at() == ua))
                                && (start_at.is_none()
                                    || start_at.map_or(false, |sa| {
                                        doc.start_at().map_or(false, |dsa| *dsa == sa)
                                    }))
                                && (end_at.is_none()
                                    || end_at.map_or(false, |ea| {
                                        doc.end_at().map_or(false, |dea| *dea == ea)
                                    }))
                        })
                        .collect();
                }

                if date_filtered_docs.is_empty() {
                    eprintln!("No documents found matching the criteria.");
                    return Ok(());
                }

                if !is_visual {
                    // 1. フィルタリングされたドキュメントをコレクション（Vec）としてまとめる
                    let collected_docs: Vec<_> = date_filtered_docs.into_iter().collect();

                    // 2. 配列全体をJSON文字列に変換する
                    let json_array = serde_json::to_string(&collected_docs)?;

                    // 3. 出力
                    println!("{}", json_array);
                } else {
                    println!(
                        "Found {} documents matching the criteria:",
                        date_filtered_docs.len()
                    );
                    // ターミナルで見やすい形式で表示
                    for doc in date_filtered_docs {
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

// レスポンス表示用の共通関数を作るとスッキリします
fn print_response(res: reqwest::blocking::Response) -> anyhow::Result<()> {
    println!("Status: {}", res.status());
    let body = res.text()?;
    println!("{}", body);
    Ok(())
}
