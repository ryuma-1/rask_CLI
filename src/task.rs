use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    print_service::{self, Printable, PrintableList},
    rask::{Creator, Project, ProjectId, Tag, Url},
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct Content {
    content: String,
}

impl Content {
    pub fn value(&self) -> &str {
        &self.content
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Assigner {
    id: AssignerId,
    name: AssignerName,
}

impl Assigner {
    #[allow(dead_code)]
    pub fn new(id: AssignerId, name: AssignerName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &AssignerId {
        &self.id
    }

    pub fn name(&self) -> &AssignerName {
        &self.name
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct TaskId {
    id: u32,
}

impl TaskId {
    pub fn value(&self) -> u32 {
        self.id
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct AssignerId {
    assigner_id: i32,
}

impl AssignerId {
    #[allow(dead_code)]
    pub fn new(s: &str) -> Result<Self> {
        let assigner_id = s
            .parse()
            .with_context(|| format!("assigner_id は数値で入力してください: '{}'", s))?;
        Ok(Self { assigner_id })
    }

    pub fn value(&self) -> i32 {
        self.assigner_id
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct AssignerName {
    name: String,
}

impl AssignerName {
    pub fn value(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct Description {
    description: String,
}

impl Description {
    #[allow(dead_code)]
    pub fn new(s: &str) -> Self {
        Self {
            description: s.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.description
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct TaskStateId {
    task_state_id: i32,
}

impl TaskStateId {
    #[allow(dead_code)]
    pub fn new(s: &str) -> Result<Self> {
        let task_state_id = s
            .parse()
            .with_context(|| format!("task_state_id は数値で入力してください: '{}'", s))?;
        Ok(Self { task_state_id })
    }

    pub fn value(&self) -> i32 {
        self.task_state_id
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct TaskReq {
    assigner_id: AssignerId,
    content: Content,
    due_at: DateTime<Utc>,
    description: Description,
    project_id: ProjectId,
    task_state_id: TaskStateId,
}

impl TaskReq {
    #[allow(dead_code)]
    pub fn new(
        assigner_id: AssignerId,
        content: Content,
        due_at: DateTime<Utc>,
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

    pub fn assigner_id(&self) -> &AssignerId {
        &self.assigner_id
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn due_at(&self) -> &DateTime<Utc> {
        &self.due_at
    }

    pub fn description(&self) -> &Description {
        &self.description
    }

    pub fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    pub fn task_state_id(&self) -> &TaskStateId {
        &self.task_state_id
    }
}

impl Printable for TaskReq {
    fn get_print_fields(&self) -> Vec<print_service::PrintField> {
        vec![
            print_service::PrintField::new("assigner_id", &self.assigner_id().value().to_string()),
            print_service::PrintField::new("content", self.content().value()),
            print_service::PrintField::new("due_at", &self.due_at().to_string()),
            print_service::PrintField::new("description", self.description().value()),
            print_service::PrintField::new("project_id", &self.project_id().value().to_string()),
            print_service::PrintField::new(
                "task_state_id",
                &self.task_state_id().value().to_string(),
            ),
        ]
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskRes {
    id: TaskId,
    content: Content,
    description: Option<Description>,
    due_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    creator: Creator,
    assigner: Assigner,
    project: Option<Project>,
    tags: Vec<Tag>,
    url: Url,
}

impl TaskRes {
    #[allow(dead_code)]
    pub fn new(
        id: TaskId,
        content: Content,
        description: Option<Description>,
        due_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        creator: Creator,
        assigner: Assigner,
        project: Option<Project>,
        tags: Vec<Tag>,
        url: Url,
    ) -> Self {
        Self {
            id,
            content,
            description,
            due_at,
            created_at,
            updated_at,
            creator,
            assigner,
            project,
            tags,
            url,
        }
    }

    pub fn id(&self) -> &TaskId {
        &self.id
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }

    pub fn due_at(&self) -> &DateTime<Utc> {
        &self.due_at
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn creator(&self) -> &Creator {
        &self.creator
    }

    pub fn assigner(&self) -> &Assigner {
        &self.assigner
    }

    pub fn project(&self) -> Option<&Project> {
        self.project.as_ref()
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}

impl Printable for TaskRes {
    fn get_print_fields(&self) -> Vec<print_service::PrintField> {
        vec![
            print_service::PrintField::new("id", &self.id().value().to_string()),
            print_service::PrintField::new("content", self.content().value()),
            print_service::PrintField::new(
                "description",
                self.description()
                    .map(|d| d.value())
                    .unwrap_or_else(|| "None"),
            ),
            print_service::PrintField::new("due_at", &self.due_at().to_string()),
            print_service::PrintField::new("created_at", &self.created_at().to_string()),
            print_service::PrintField::new("updated_at", &self.updated_at().to_string()),
            print_service::PrintField::new("creator_id", &self.creator().id().value().to_string()),
            print_service::PrintField::new("creator_name", self.creator().name().value()),
            print_service::PrintField::new(
                "assigner_id",
                &self.assigner().id().value().to_string(),
            ),
            print_service::PrintField::new("assigner_name", self.assigner().name().value()),
            print_service::PrintField::new(
                "project_name",
                self.project()
                    .map(|p| p.name().value())
                    .unwrap_or_else(|| "None"),
            ),
            print_service::PrintField::new(
                "tags",
                if self.tags().is_empty() {
                    "None".to_string()
                } else {
                    self.tags()
                        .iter()
                        .map(|t| t.name().value())
                        .collect::<Vec<_>>()
                        .join(", ")
                },
            ),
            print_service::PrintField::new("url", self.url().value()),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct TaskResList {
    tasks: Vec<TaskRes>,
}

impl TaskResList {
    pub fn new(tasks: Vec<TaskRes>) -> Self {
        Self { tasks }
    }

    pub fn tasks(&self) -> &Vec<TaskRes> {
        &self.tasks
    }
}

impl PrintableList for TaskResList {
    fn get_printable_list(&self) -> Vec<Box<dyn Printable>> {
        self.tasks()
            .iter()
            .map(|task| Box::new(task.clone()) as Box<dyn Printable>)
            .collect()
    }
}
