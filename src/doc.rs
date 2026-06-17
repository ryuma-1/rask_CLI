use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::{
    print_service::{self, Printable, PrintableList},
    rask::{Creator, Project, ProjectId, Tag, Url},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
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
#[serde(transparent)]
pub struct Description {
    description: String,
}

impl Description {
    pub fn value(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct DocId {
    id: u32,
}

impl DocId {
    pub fn value(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct Location {
    location: String,
}

impl Location {
    pub fn value(&self) -> &str {
        &self.location
    }
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Display)]
pub enum DocType {
    New,
    GN,
    Other,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocReq {
    content: Content,
    description: Description,
    project_id: ProjectId,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
    location: Location,
}

#[allow(dead_code)]
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

impl Printable for DocReq {
    fn get_print_fields(&self) -> Vec<print_service::PrintField> {
        vec![
            print_service::PrintField::new("content", self.content().value()),
            print_service::PrintField::new("description", self.description().value()),
            print_service::PrintField::new("project_id", &self.project_id().value().to_string()),
            print_service::PrintField::new("start_at", &self.start_at().to_string()),
            print_service::PrintField::new("end_at", &self.end_at().to_string()),
            print_service::PrintField::new("location", self.location().value()),
        ]
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    url: Url,
}

impl DocRes {
    #[allow(dead_code)]
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
        url: Url,
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
        self.start_at.as_ref()
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

    pub fn url(&self) -> &Url {
        &self.url
    }
}

impl Printable for DocRes {
    fn get_print_fields(&self) -> Vec<print_service::PrintField> {
        vec![
            print_service::PrintField::new("id", &self.id().value().to_string()),
            print_service::PrintField::new("content", self.content().value()),
            print_service::PrintField::new("creator_id", &self.creator().id().value().to_string()),
            print_service::PrintField::new("creator_name", self.creator().name().value()),
            print_service::PrintField::new(
                "description",
                self.description()
                    .map(|d| d.value())
                    .unwrap_or_else(|| "None"),
            ),
            print_service::PrintField::new("created_at", &self.created_at().to_string()),
            print_service::PrintField::new("updated_at", &self.updated_at().to_string()),
            print_service::PrintField::new(
                "project_name",
                self.project()
                    .map(|p| p.name().value())
                    .unwrap_or_else(|| "None"),
            ),
            print_service::PrintField::new(
                "start_at",
                self.start_at()
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "None".to_string()),
            ),
            print_service::PrintField::new(
                "end_at",
                self.end_at()
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "None".to_string()),
            ),
            print_service::PrintField::new(
                "location",
                self.location().map(|l| l.value()).unwrap_or_else(|| "None"),
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
pub struct DocResList {
    docs: Vec<DocRes>,
}

impl DocResList {
    pub fn new(docs: Vec<DocRes>) -> Self {
        Self { docs }
    }

    pub fn docs(&self) -> &Vec<DocRes> {
        &self.docs
    }

    pub fn filter_by_id(&self, id: u32) -> Self {
        let filtered = self
            .docs
            .iter()
            .filter(|doc| id == doc.id().value())
            .cloned()
            .collect();
        Self::new(filtered)
    }

    pub fn filter_by_content(&self, content: &[String]) -> Self {
        if content.is_empty() {
            return Self::new(self.docs.clone());
        }
        let filtered = self
            .docs
            .iter()
            .filter(|doc| content.iter().all(|kw| doc.content().value().contains(kw)))
            .cloned()
            .collect();
        Self::new(filtered)
    }

    pub fn filter_by_creator(&self, creator_id: Option<u32>, creator_name: &[String]) -> Self {
        let filtered = self
            .docs
            .iter()
            .filter(|doc| {
                let match_id =
                    creator_id.is_none() || creator_id == Some(doc.creator().id().value());

                let match_name = creator_name.is_empty()
                    || creator_name
                        .iter()
                        .all(|kw| doc.creator().name().value().contains(kw));

                match_id && match_name
            })
            .cloned()
            .collect();
        Self::new(filtered)
    }

    pub fn filter_by_description(&self, description: &[String]) -> Self {
        if description.is_empty() {
            return Self::new(self.docs.clone());
        }
        let filtered = self
            .docs
            .iter()
            .filter(|doc| {
                description
                    .iter()
                    .all(|kw| doc.description().is_some_and(|d| d.value().contains(kw)))
            })
            .cloned()
            .collect();
        Self::new(filtered)
    }

    pub fn filter_by_project(&self, project_id: Option<u32>, project_name: &[String]) -> Self {
        let filtered = self
            .docs
            .iter()
            .filter(|doc| {
                let match_id = project_id.is_none()
                    || doc
                        .project()
                        .is_some_and(|p| project_id == Some(p.id().value()));

                let match_name = project_name.is_empty()
                    || project_name
                        .iter()
                        .all(|kw| doc.project().is_some_and(|p| p.name().value().contains(kw)));

                match_id && match_name
            })
            .cloned()
            .collect();
        Self::new(filtered)
    }

    pub fn filter_by_date_range(
        &self,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
        start_at: Option<DateTime<Utc>>,
        end_at: Option<DateTime<Utc>>,
        term_day: u32,
    ) -> Self {
        let term_duration = chrono::Duration::days(term_day as i64);

        let filtered = self
            .docs
            .iter()
            .filter(|doc| {
                let within_created_at = created_at.is_none()
                    || created_at.is_some_and(|ca| {
                        let lower = ca - term_duration;
                        let upper = ca + term_duration;
                        lower <= *doc.created_at() && *doc.created_at() <= upper
                    });

                let within_updated_at = updated_at.is_none()
                    || updated_at.is_some_and(|ua| {
                        let lower = ua - term_duration;
                        let upper = ua + term_duration;
                        lower <= *doc.updated_at() && *doc.updated_at() <= upper
                    });

                let within_start_at = start_at.is_none()
                    || start_at.is_some_and(|sa| {
                        doc.start_at().copied().is_some_and(|doc_start| {
                            let lower = sa - term_duration;
                            let upper = sa + term_duration;
                            lower <= doc_start && doc_start <= upper
                        })
                    });

                let within_end_at = end_at.is_none()
                    || end_at.is_some_and(|ea| {
                        doc.end_at().copied().is_some_and(|doc_end| {
                            let lower = ea - term_duration;
                            let upper = ea + term_duration;
                            lower <= doc_end && doc_end <= upper
                        })
                    });

                within_created_at && within_updated_at && within_start_at && within_end_at
            })
            .cloned()
            .collect();
        Self::new(filtered)
    }

    pub fn filter_by_date_exact(
        &self,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
        start_at: Option<DateTime<Utc>>,
        end_at: Option<DateTime<Utc>>,
    ) -> Self {
        let filtered = self
            .docs
            .iter()
            .filter(|doc| {
                let match_created_at =
                    created_at.is_none() || created_at.is_some_and(|ca| *doc.created_at() == ca);

                let match_updated_at =
                    updated_at.is_none() || updated_at.is_some_and(|ua| *doc.updated_at() == ua);

                let match_start_at = start_at.is_none()
                    || start_at.is_some_and(|sa| doc.start_at().is_some_and(|dsa| *dsa == sa));

                let match_end_at = end_at.is_none()
                    || end_at.is_some_and(|ea| doc.end_at().is_some_and(|dea| *dea == ea));

                match_created_at && match_updated_at && match_start_at && match_end_at
            })
            .cloned()
            .collect();
        Self::new(filtered)
    }
}

impl PrintableList for DocResList {
    fn get_printable_list(&self) -> Vec<Box<dyn Printable>> {
        self.docs()
            .iter()
            .map(|doc| Box::new(doc.clone()) as Box<dyn Printable>)
            .collect()
    }
}
