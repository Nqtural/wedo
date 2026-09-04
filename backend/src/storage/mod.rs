use crate::types::{List, ListOverview, ListState, Task, TaskOverview, TaskState};
use async_trait::async_trait;
use uuid::Uuid;

pub mod sqlite;

#[derive(Debug)]
pub enum StorageError {
	NotFound,
	Database(sqlx::Error),
	Uuid(uuid::Error),
}

impl From<uuid::Error> for StorageError {
	fn from(error: uuid::Error) -> Self {
		StorageError::Uuid(error)
	}
}

impl From<sqlx::Error> for StorageError {
	fn from(error: sqlx::Error) -> Self {
		StorageError::Database(error)
	}
}

#[async_trait]
pub trait Storage: Send + Sync + 'static {
	// lists
	async fn create_list(&self, state: ListState) -> Result<List, StorageError>;
	async fn get_list_overview(&self) -> Result<Vec<ListOverview>, StorageError>;
	async fn get_list(&self, list_id: Uuid) -> Result<ListOverview, StorageError>;
	async fn update_list(&self, list_id: Uuid, state: ListState) -> Result<List, StorageError>;
	async fn delete_list(&self, list_id: Uuid) -> Result<(), StorageError>;

	// tasks
	async fn create_task(&self, list_id: Uuid, state: TaskState) -> Result<Task, StorageError>;
	async fn get_task_overview(&self, list_id: Uuid) -> Result<Vec<TaskOverview>, StorageError>;
	async fn get_task(&self, task_id: Uuid) -> Result<Task, StorageError>;
	async fn update_task(&self, task_id: Uuid, state: TaskState) -> Result<Task, StorageError>;
	async fn delete_task(&self, task_id: Uuid) -> Result<(), StorageError>;
}
