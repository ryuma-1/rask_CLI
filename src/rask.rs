use serde::{Deserialize, Serialize};

// ============================================================
// Shared newtype structs
// ============================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    id: ProjectId,
    name: ProjectName,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    id: TagId,
    name: TagName,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct Url {
    url: String,
}

// ============================================================
// Shared nested structs
// ============================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Creator {
    id: CreatorId,
    name: CreatorName,
}

// ============================================================
// impl CreatorId / CreatorName
// ============================================================

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

// ============================================================
// impl ProjectName
// ============================================================

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

// ============================================================
// impl TagName
// ============================================================

impl TagId {
    #[allow(dead_code)]
    pub fn value(&self) -> u32 {
        self.id
    }
}

impl TagName {
    pub fn value(&self) -> &str {
        &self.name
    }
}

// ============================================================
// impl Creator
// ============================================================

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

impl Url {
    pub fn value(&self) -> &str {
        &self.url
    }
}
