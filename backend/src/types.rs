use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
	pub id: Uuid,
	pub items: Vec<Task>,
	pub state: ListState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListState {
	pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOverview {
	pub id: Uuid,
	pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
	pub id: Uuid,
	pub state: TaskState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskState {
	pub name: String,
	pub description: String,
	pub completed: bool,
	pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskOverview {
	pub id: Uuid,
	pub name: String,
	pub completed: bool,
	pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
	pub id: Uuid,
	pub username: String,
	pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinResult {
	pub list: ListOverview,
	pub joined: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
	pub id: Uuid,
	pub state: TagState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagState {
	pub name: String,
	pub color_key: String,
}
