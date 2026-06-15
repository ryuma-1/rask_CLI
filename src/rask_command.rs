use crate::doc::*;
use crate::rask_api::*;

use anyhow::{Context, Ok, Result};
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
                // 1. まずはすべてのドキュメントを取得
                let res = rask_api.get_all_docs()?;

                // 2. レスポンスを Vec<DocRes> に変換
                let doc_res: Vec<DocRes> = serde_json::from_str(&res.text()?)?;

                let result_res: Vec<DocRes>;

                if id.is_none() {
                    // 3. id が指定されている場合は，id で絞り込み
                    let id_filtered_docs: Vec<DocRes> = doc_res
                        .into_iter()
                        .filter(|doc| id.is_none() || id == Some(doc.id().value()))
                        .collect();
                    result_res = id_filtered_docs;
                } else {
                    // 各フィールドで絞り込み
                    let filtered_docs: Vec<DocRes> = doc_res
                        .into_iter()
                        .filter(|doc| {
                        // 1. 本文（コンテンツ）のAND検索
                            (content.is_empty()
                            || doc.content().is_match(&content))

                        // 2. 作成者IDの一致（Option型同士の比較はシンプルにするのが一番！）
                        && (creator_id.is_none()
                            || creator_id == Some(doc.creator().id().value()))

                        // 3. 作成者名のAND検索
                        && (creator_name.is_empty()
                            || creator_name.iter().all(|kw| doc.creator().name().value().contains(kw)))

                        // 4. 概要（Description）のAND検索（Option型なのでmap_orで安全に守る）
                        && (description.is_empty()
                            || description.iter().all(|kw| {
                                doc.description().is_some_and(|d| d.value().contains(kw))
                            }))

                        // 5. プロジェクトIDの一致（プロジェクトが無い場合は0を身代わりにするか、以下のようにmap判定）
                        && (project_id.is_none()
                            || doc.project().is_some_and(|p| project_id == Some(p.id().value())))

                        // 6. プロジェクト名のAND検索（プロジェクトが無い場合はfalse）
                        && (project_name.is_empty()
                            || project_name.iter().all(|kw| {
                                doc.project().is_some_and(|p| p.name().value().contains(kw))
                            }))
                        })
                        .collect( );

                    // 4. 日付フィールドでさらに絞り込み
                    if let Some(term) = term_day {
                        // term_day が指定されている場合：各日付フィールドを「±term_duration の範囲内」でフィルタリング
                        let term_duration = chrono::Duration::days(term as i64);

                        let date_filtered_docs = filtered_docs
                            .into_iter()
                            .filter(|doc| {
                                // created_at が未指定、または doc の created_at が [ca - term, ca + term] の範囲内
                                let within_created_at = created_at.is_none()
                                    || created_at.is_some_and(|ca| {
                                        let lower = ca - term_duration;
                                        let upper = ca + term_duration;
                                        lower <= *doc.created_at() && *doc.created_at() <= upper
                                    });

                                // updated_at が未指定、または doc の updated_at が [ua - term, ua + term] の範囲内
                                let within_updated_at = updated_at.is_none()
                                    || updated_at.is_some_and(|ua| {
                                        let lower = ua - term_duration;
                                        let upper = ua + term_duration;
                                        lower <= *doc.updated_at() && *doc.updated_at() <= upper
                                    });

                                // start_at が未指定、または doc の start_at が [sa - term, sa + term] の範囲内
                                // doc.start_at() が None の場合は Default値（epoch）で比較
                                let within_start_at = start_at.is_none()
                                    || start_at.is_some_and(|sa| {
                                        let doc_start = doc.start_at().copied().unwrap_or_default();
                                        let lower = sa - term_duration;
                                        let upper = sa + term_duration;
                                        lower <= doc_start && doc_start <= upper
                                    });

                                // end_at が未指定、または doc の end_at が [ea - term, ea + term] の範囲内
                                // doc.end_at() が None の場合は unwrap() でパニックする点に注意
                                let within_end_at = end_at.is_none()
                                    || end_at.is_some_and(|ea| {
                                        let doc_end = doc.end_at().copied().unwrap(); // ※ None の場合パニック
                                        let lower = ea - term_duration;
                                        let upper = ea + term_duration;
                                        lower <= doc_end && doc_end <= upper
                                    });

                                within_created_at
                                    && within_updated_at
                                    && within_start_at
                                    && within_end_at
                            })
                            .collect();

                        result_res = date_filtered_docs;
                    } else {
                        // term_day が未指定の場合：各日付フィールドを「完全一致」でフィルタリング
                        let date_filtered_docs = filtered_docs
                            .into_iter()
                            .filter(|doc| {
                                // created_at が未指定、または doc の created_at と完全一致
                                let match_created_at = created_at.is_none()
                                    || created_at.is_some_and(|ca| *doc.created_at() == ca);

                                // updated_at が未指定、または doc の updated_at と完全一致
                                let match_updated_at = updated_at.is_none()
                                    || updated_at.is_some_and(|ua| *doc.updated_at() == ua);

                                // start_at が未指定、または doc の start_at が Some かつ完全一致
                                let match_start_at = start_at.is_none()
                                    || start_at.is_some_and(|sa| {
                                        doc.start_at().is_some_and(|dsa| *dsa == sa)
                                    });

                                // end_at が未指定、または doc の end_at が Some かつ完全一致
                                let match_end_at = end_at.is_none()
                                    || end_at.is_some_and(|ea| {
                                        doc.end_at().is_some_and(|dea| *dea == ea)
                                    });

                                match_created_at
                                    && match_updated_at
                                    && match_start_at
                                    && match_end_at
                            })
                            .collect();

                        result_res = date_filtered_docs;
                    }
                }

                // 4. 各フィールドで絞り込み
                if result_res.is_empty() {
                    eprintln!("No documents found matching the criteria.");
                    return Ok(());
                }

                if !is_visual {
                    // 1. フィルタリングされたドキュメントをコレクション（Vec）としてまとめる
                    let collected_docs: Vec<_> = result_res.into_iter().collect();

                    // 2. 配列全体をJSON文字列に変換する
                    let json_array = serde_json::to_string(&collected_docs)?;

                    // 3. 出力
                    println!("{}", json_array);
                } else {
                    println!(
                        "Found {} documents matching the criteria:",
                        result_res.len()
                    );
                    // ターミナルで見やすい形式で表示
                    for doc in result_res {
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
