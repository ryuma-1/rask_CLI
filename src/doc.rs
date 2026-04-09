use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::date::Date;
use crate::task::{Content, Description, FromString, ProjectId};

#[derive(Deserialize, Serialize, Debug)]
pub struct DocReq {
	#[serde(rename = "document")]
	document: Doc,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Doc {
	content: Content,
	description: Description,
	project_id: ProjectId,
	start_at: StartAt,
	end_at: EndAt,
	location: Location,
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
	pub fn new(document: Doc) -> Self {
		Self { document }
	}
}

impl Doc {
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
