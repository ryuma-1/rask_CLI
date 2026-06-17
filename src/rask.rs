use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct CreatorId {
    id: u32,
}

impl CreatorId {
    pub fn value(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct CreatorName {
    name: String,
}

impl CreatorName {
    pub fn value(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct ProjectId {
    id: u32,
}

impl ProjectId {
    pub fn value(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct ProjectName {
    name: String,
}

impl ProjectName {
    pub fn value(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct TagId {
    id: u32,
}

impl TagId {
    #[allow(dead_code)]
    pub fn value(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct TagName {
    name: String,
}

impl TagName {
    pub fn value(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct Url {
    url: String,
}

impl Url {
    pub fn value(&self) -> &str {
        &self.url
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Creator {
    id: CreatorId,
    name: CreatorName,
}

impl Creator {
    #[allow(dead_code)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    id: ProjectId,
    name: ProjectName,
}

impl Project {
    #[allow(dead_code)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    id: TagId,
    name: TagName,
}

impl Tag {
    #[allow(dead_code)]
    pub fn new(id: TagId, name: TagName) -> Self {
        Self { id, name }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> &TagId {
        &self.id
    }

    pub fn name(&self) -> &TagName {
        &self.name
    }
}
