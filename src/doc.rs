#![allow(dead_code)]

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Deserialize, Serialize)]
pub struct DocRes {
    id: DocId,
    content: Content,
    creator: Creator,
    description: Option<Description>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    project: Option<Project>,
    start_at: Option<DateTime<Utc>>,
    end_at: Option<DateTime<Utc>>,
    location: Option<Location>,
    tags: Vec<Tag>,
    url: DocUrl,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DocReq {
    content: Content,
    description: Description,
    project_id: ProjectId,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
    location: Location,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Display)]
pub enum DocType {
    New,
    GN,
    Other,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct Content {
    content: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct Description {
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Creator {
    id: CreatorId,
    name: CreatorName,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    id: ProjectId,
    name: ProjectName,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    id: TagId,
    name: TagName,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct DocId {
    id: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct DocUrl {
    url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct CreatorId {
    id: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct CreatorName {
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct ProjectId {
    id: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct ProjectName {
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct TagId {
    id: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct TagName {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct Location {
    location: String,
}

impl DocReq {
    pub fn new(
        content: Content,
        description: Description,
        project_id: ProjectId,
        start_at: DateTime<Utc>,
        end_at: DateTime<Utc>,
        location: Location,
    ) -> Self {
        Self {
            content,
            description,
            project_id,
            start_at,
            end_at,
            location,
        }
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn description(&self) -> &Description {
        &self.description
    }

    pub fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    pub fn start_at(&self) -> &DateTime<Utc> {
        &self.start_at
    }

    pub fn end_at(&self) -> &DateTime<Utc> {
        &self.end_at
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

impl DocRes {
    pub fn new(
        id: DocId,
        content: Content,
        creator: Creator,
        description: Option<Description>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        project: Option<Project>,
        start_at: Option<DateTime<Utc>>,
        end_at: Option<DateTime<Utc>>,
        location: Option<Location>,
        tags: Vec<Tag>,
        url: DocUrl,
    ) -> Self {
        Self {
            id,
            content,
            creator,
            description,
            created_at,
            updated_at,
            project,
            start_at,
            end_at,
            location,
            tags,
            url,
        }
    }

    pub fn to_type(&self) -> DocType {
        // contentの文字列からGNまたはNewという文字列が存在するかを確認
        let content_str = self.content().value();
        if content_str.contains("GN") {
            DocType::GN
        } else if content_str.contains("New") {
            DocType::New
        } else {
            // デフォルトはNewとする
            DocType::Other
        }
    }

    pub fn id(&self) -> &DocId {
        &self.id
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn creator(&self) -> &Creator {
        &self.creator
    }

    pub fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn project(&self) -> Option<&Project> {
        self.project.as_ref()
    }

    pub fn start_at(&self) -> Option<&DateTime<Utc>> {
        match self.start_at {
            Some(ref start_at) => Some(start_at),
            None => None,
        }
    }

    pub fn end_at(&self) -> Option<&DateTime<Utc>> {
        self.end_at.as_ref()
    }

    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn url(&self) -> &DocUrl {
        &self.url
    }
}

impl Content {
    pub fn is_match(&self, keywords: &[String]) -> bool {
        keywords.iter().all(|kw| self.content.contains(kw))
    }

    pub fn value(&self) -> String {
        self.content.clone()
    }
}

impl Creator {
    pub fn new(id: CreatorId, name: CreatorName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &CreatorId {
        &self.id
    }

    pub fn name(&self) -> &CreatorName {
        &self.name
    }
}

impl Project {
    pub fn new(id: ProjectId, name: ProjectName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &ProjectId {
        &self.id
    }

    pub fn name(&self) -> &ProjectName {
        &self.name
    }
}

impl Description {
    pub fn value(&self) -> String {
        self.description.clone()
    }
}

impl Tag {
    pub fn new(id: TagId, name: TagName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &TagId {
        &self.id
    }

    pub fn name(&self) -> &TagName {
        &self.name
    }
}

impl DocId {
    pub fn value(&self) -> u32 {
        self.id
    }
}

impl DocUrl {
    pub fn value(&self) -> &str {
        &self.url
    }

    pub fn trim_json(&self) -> String {
        self.url
            .strip_suffix(".json")
            .unwrap_or(&self.url)
            .to_string()
    }
}

impl CreatorId {
    pub fn value(&self) -> u32 {
        self.id
    }
}

impl CreatorName {
    pub fn value(&self) -> &str {
        &self.name
    }
}

impl ProjectId {
    pub fn value(&self) -> u32 {
        self.id
    }
}

impl ProjectName {
    pub fn value(&self) -> &str {
        &self.name
    }
}

impl TagId {
    pub fn value(&self) -> u32 {
        self.id
    }
}

impl TagName {
    pub fn value(&self) -> &str {
        &self.name
    }
}

impl Location {
    pub fn value(&self) -> String {
        self.location.clone()
    }
}
