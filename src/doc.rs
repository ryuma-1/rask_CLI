use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use strum::Display;


use crate::date::Date;
use crate::task::{Content, Description, FromString, ProjectId};

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
	start_at: StartAt,
	end_at: EndAt,
	location: Location,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Display)]
pub enum DocType{
	New,
	GN,
	Other
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Creator {
	id: CreatorId,
	name: CreatorName,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
	id: ProjectResId,
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
	id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct DocUrl {
	url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct CreatorId {
	id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct CreatorName {
	name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct ProjectResId {
	id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct ProjectName {
	name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct TagId {
	id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct TagName {
	name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct StartAt {
	start_at: Date,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]	
pub struct EndAt {
	end_at: Date,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct Location {
	location: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct Tags {
	tags: Vec<String>,
}


impl DocReq {
	pub fn new(
		content: Content,
		description: Description,
		project_id: ProjectId,
		start_at: StartAt,
		end_at: EndAt,
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

	pub fn start_at(&self) -> &StartAt {
		&self.start_at
	}

	pub fn end_at(&self) -> &EndAt {
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

	pub fn url(&self) -> &DocUrl {
		&self.url
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
	pub fn new(id: ProjectResId, name: ProjectName) -> Self {
		Self { id, name }
	}

	pub fn id(&self) -> &ProjectResId {
		&self.id
	}

	pub fn name(&self) -> &ProjectName {
		&self.name
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
	pub fn value(&self) -> i32 {
		self.id
	}
}

impl DocUrl {
	pub fn value(&self) -> &str {
		&self.url
	}

	pub fn trim_json(&self) -> String {
		self.url.strip_suffix(".json").unwrap_or(&self.url).to_string()
	}

}

impl CreatorId {
	pub fn value(&self) -> i32 {
		self.id
	}

}

impl CreatorName {
	pub fn value(&self) -> &str {
		&self.name
	}
}

impl ProjectResId {
	pub fn value(&self) -> i32 {
		self.id
	}
}

impl ProjectName {
	pub fn value(&self) -> &str {
		&self.name
	}
}

impl TagId {
	pub fn value(&self) -> i32 {
		self.id
	}
}

impl TagName {
	pub fn value(&self) -> &str {
		&self.name
	}
}

fn parse_date(s: &String, field_name: &str) -> Result<Date, String> {
	let parts: Vec<&str> = s.split('-').collect();
	if parts.len() != 3 {
		return Err(format!("{} は YYYY-MM-DD 形式で入力してください。", field_name));
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

	Date::new(year, month, day).map_err(|e| e.to_string())
}

impl StartAt {
	pub fn value(&self) -> Date {
		self.start_at.clone()
	}
}

impl FromString for StartAt {
	fn new(s: &String) -> Result<Self, String> {
		let start_at = parse_date(s, "start_at")?;
		Ok(Self { start_at })
	}

	fn to_string(&self) -> String {
		self.start_at.to_string()
	}
}

impl EndAt {
	pub fn value(&self) -> Date {
		self.end_at.clone()
	}
}

impl FromString for EndAt {
	fn new(s: &String) -> Result<Self, String> {
		let end_at = parse_date(s, "end_at")?;
		Ok(Self { end_at })
	}

	fn to_string(&self) -> String {
		self.end_at.to_string()
	}
}

impl Location {
	pub fn value(&self) -> String {
		self.location.clone()
	}
}

impl FromString for Location {
	fn new(s: &String) -> Result<Self, String> {
		Ok(Self { location: s.clone() })
	}

	fn to_string(&self) -> String {
		self.location.clone()
	}
}

impl Tags {
	pub fn value(&self) -> Vec<String> {
		self.tags.clone()
	}
}

impl FromString for Tags {
	fn new(s: &String) -> Result<Self, String> {
		let tags = if s.trim().is_empty() {
			Vec::new()
		} else {
			s.split(',')
				.map(|tag| tag.trim())
				.filter(|tag| !tag.is_empty())
				.map(|tag| tag.to_string())
				.collect()
		};

		Ok(Self { tags })
	}

	fn to_string(&self) -> String {
		self.tags.join(",")
	}
}
