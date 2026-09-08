use crate::types::{
	Account, Credentials, JoinResult, List, ListOverview, ListState, Tag, TagState, Task,
	TaskOverview, TaskState,
};
use async_trait::async_trait;
use uuid::Uuid;

pub mod sqlite;

pub enum AuthError {
	Forbidden,
	InvalidCredentials,
	Storage(StorageError),
	Hash(bcrypt::BcryptError),
}

impl From<StorageError> for AuthError {
	fn from(error: StorageError) -> Self {
		AuthError::Storage(error)
	}
}

impl From<bcrypt::BcryptError> for AuthError {
	fn from(error: bcrypt::BcryptError) -> Self {
		AuthError::Hash(error)
	}
}

#[derive(Debug)]
pub enum StorageError {
	NotFound,
	Conflict,
	Database(sqlx::Error),
	Uuid(uuid::Error),
	Hash(bcrypt::BcryptError),
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

impl From<bcrypt::BcryptError> for StorageError {
	fn from(error: bcrypt::BcryptError) -> Self {
		StorageError::Hash(error)
	}
}

#[async_trait]
pub trait Storage: Send + Sync + 'static {
	// lists
	async fn create_list(&self, account_id: Uuid, state: ListState) -> Result<List, StorageError>;
	async fn get_list_overview(&self, account_id: Uuid) -> Result<Vec<ListOverview>, StorageError>;
	async fn get_list(&self, account_id: Uuid, list_id: Uuid)
	-> Result<ListOverview, StorageError>;
	async fn update_list(
		&self,
		account_id: Uuid,
		list_id: Uuid,
		state: ListState,
	) -> Result<List, StorageError>;
	async fn delete_list(&self, account_id: Uuid, list_id: Uuid) -> Result<(), StorageError>;

	// tasks
	async fn create_task(
		&self,
		account_id: Uuid,
		list_id: Uuid,
		state: TaskState,
	) -> Result<Task, StorageError>;
	async fn get_task_overview(
		&self,
		account_id: Uuid,
		list_id: Uuid,
	) -> Result<Vec<TaskOverview>, StorageError>;
	async fn get_task(&self, account_id: Uuid, task_id: Uuid) -> Result<Task, StorageError>;
	async fn update_task(
		&self,
		account_id: Uuid,
		task_id: Uuid,
		state: TaskState,
	) -> Result<Task, StorageError>;
	async fn delete_task(&self, account_id: Uuid, task_id: Uuid) -> Result<(), StorageError>;

	// accounts
	async fn create_account(&self, credentials: &Credentials) -> Result<(), StorageError>;
	async fn get_account(&self, account_id: Uuid) -> Result<Account, StorageError>;
	async fn delete_account(&self, account_id: Uuid) -> Result<(), StorageError>;

	// sessions
	async fn create_session(&self, credentials: &Credentials) -> Result<Uuid, AuthError>;
	async fn validate_session(&self, session_id: Uuid) -> Result<Uuid, StorageError>;
	async fn delete_session(&self, session_id: Uuid) -> Result<(), StorageError>;

	// sharing
	async fn create_invitation(&self, account_id: Uuid, list_id: Uuid)
	-> Result<String, AuthError>;
	async fn accept_invitation(
		&self,
		account_id: Uuid,
		invitation_id: String,
	) -> Result<JoinResult, StorageError>;

	// tags
	async fn create_and_apply_tag(
		&self,
		account_id: Uuid,
		task_id: Uuid,
		tag_state: TagState,
	) -> Result<Tag, StorageError>;
	async fn update_tag(
		&self,
		account_id: Uuid,
		tag_id: Uuid,
		tag_state: TagState,
	) -> Result<Tag, StorageError>;
	async fn apply_tag(
		&self,
		account_id: Uuid,
		tag_id: Uuid,
		task_id: Uuid,
	) -> Result<Tag, StorageError>;
	async fn remove_tag(
		&self,
		account_id: Uuid,
		tag_id: Uuid,
		task_id: Uuid,
	) -> Result<(), StorageError>;
	async fn delete_tag(&self, account_id: Uuid, tag_id: Uuid) -> Result<(), StorageError>;
}
