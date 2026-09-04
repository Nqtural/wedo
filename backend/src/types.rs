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

#[derive(Debug, Clone, Serialize)]
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
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskOverview {
	pub id: Uuid,
	pub name: String,
	pub completed: bool,
}
