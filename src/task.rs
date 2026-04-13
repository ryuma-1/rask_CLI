use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::{date::Date, doc::DocType, minute::MinuteType};

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskReq {

    #[serde(rename = "task")]
    task: Task,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    assigner_id: AssignerId,
    content: Content,
    due_at: DueAt,
    description: Description,
    project_id: ProjectId,
    task_state_id: TaskStateId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct AssignerId {
    assigner_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct Content {
    content: String,
}

 #[derive(Deserialize, Serialize, Debug)]
 #[serde(transparent)]
pub struct DueAt {
    due_at: Date,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct Description {
    description: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct ProjectId {
    project_id: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct TaskStateId {
    task_state_id: i32,
}

impl TaskReq {
    pub fn new(task: Task) -> Self {
        Self { task }
    }
}

impl Task {
    pub fn new(
        assigner_id: AssignerId,
        content: Content,
        due_at: DueAt,
        description: Description,
        project_id: ProjectId,
        task_state_id: TaskStateId,
    ) -> Self {
        Self {
            assigner_id,
            content,
            due_at,
            description,
            project_id,
            task_state_id,
        }
    }
}

pub trait FromString: Sized {
    // 1. エラー型を指定し、Sized制約（またはSelf: Sized）が必要
    fn new(s: &String) -> Result<Self, String>;

    // 2. インスタンスメソッドにするために &self を追加
    fn to_string(&self) -> String;
}

impl AssignerId {
    pub fn value(&self) -> i32 {
        self.assigner_id
    }
}

impl FromString for AssignerId {
    fn new(s: &String) -> Result<Self, String> {
        let assigner_id = s.parse().map_err(|_| "assigner_id は数値で入力してください。")?;
        Ok(Self { assigner_id })
    }

    fn to_string(&self) -> String {
        self.assigner_id.to_string()
    }
}


impl Content {
    pub fn value(&self) -> String {
        self.content.clone()
    }

    pub fn to_type(&self) -> MinuteType {
            let content_str = self.content.clone();
            if content_str.contains(MinuteType::GN.to_string().as_str()) {
                MinuteType::GN
            } else if content_str.contains(MinuteType::New.to_string().as_str()) {
                MinuteType::New
            } else {
                // デフォルトはNewとする
                MinuteType::Other
            }
    }
}


impl FromString for Content {
    fn new(s: &String) -> Result<Self, String> {
        Ok(Self { content: s.clone() })
    }

    fn to_string(&self) -> String {
        self.content.clone()
    }
}

impl DueAt {

}

impl FromString for DueAt {
    fn new(s: &String) -> Result<Self, String> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err("due_at は YYYY-MM-DD 形式で入力してください。".to_string());
        }

        let year: u16 = parts[0]
            .parse()
            .map_err(|_| "年は数値で入力してください。".to_string())?;
        let month: u8 = parts[1]
            .parse()
            .map_err(|_| "月は数値で入力してください。".to_string())?;
        let day: u8 = parts[2]
            .parse()
            .map_err(|_| "日は数値で入力してください。".to_string())?;

        let due_at = Date::new(year, month, day).map_err(|e| e.to_string())?;
        Ok(Self { due_at })
    }

    fn to_string(&self) -> String {
        self.due_at.to_string()
    }
}

impl Description {
    pub fn value(&self) -> String {
        self.description.clone()
    }
}

impl FromString for Description {
    fn new(s: &String) -> Result<Self, String> {
        Ok(Self { description: s.clone() })
    }

    fn to_string(&self) -> String {
        self.description.clone()
    }
}

impl ProjectId {
    pub fn value(&self) -> Option<i32> {
        self.project_id
    }
}

impl FromString for ProjectId {
    fn new(s: &String) -> Result<Self, String> {
    // 1. 文字列が空（または空白のみ）なら None、値があればパースを試みる
    let project_id = if s.trim().is_empty() {
        None
    } else {
        // 2. 数値としてパースし、失敗したらエラーメッセージを返す
        let val = s.parse::<i32>()
            .map_err(|_| "project_id は数値で入力してください。")?;
        Some(val)
    };

    Ok(Self { project_id })
}

    fn to_string(&self) -> String {
        self.project_id.map_or_else(|| "".into(), |v| v.to_string())
    }
}

impl TaskStateId {
    pub fn value(&self) -> i32 {
        self.task_state_id
    }
}

impl FromString for TaskStateId {
    fn new(s: &String) -> Result<Self, String> {
        let task_state_id = s.parse().map_err(|_| "task_state_id は数値で入力してください。")?;
        Ok(Self { task_state_id })
    }

    fn to_string(&self) -> String {
        self.task_state_id.to_string()
    }
}

