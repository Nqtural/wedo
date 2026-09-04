use async_trait::async_trait;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use uuid::Uuid;

use crate::storage::{Storage, StorageError};
use crate::types::{List, ListOverview, ListState, Task, TaskOverview, TaskState};

pub struct SqliteStorage {
	pool: SqlitePool,
}

impl SqliteStorage {
	pub async fn new(path: &str) -> Result<Self, sqlx::Error> {
		let pool = SqlitePoolOptions::new()
			.max_connections(5)
			.connect(path)
			.await?;

		Ok(Self { pool })
	}

	async fn get_list(&self, list_id: Uuid) -> Result<List, StorageError> {
		let list_id_string = list_id.to_string();

		let name = sqlx::query!(
			r#"
			SELECT name FROM lists
			WHERE id = ?
			"#,
			list_id_string
		)
		.fetch_one(&self.pool)
		.await
		.map_err(StorageError::Database)?
		.name;

		let record = sqlx::query!(
			r#"
			SELECT * FROM tasks
			WHERE list_id = ?
			"#,
			list_id_string
		)
		.fetch_all(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(List {
			id: list_id,
			items: record
				.iter()
				.map(|r| {
					Ok(Task {
						id: Uuid::parse_str(&r.id)?,
						state: TaskState {
							name: r.name.clone(),
							description: r.description.clone(),
							completed: r.completed,
						},
					})
				})
				.collect::<Result<Vec<_>, StorageError>>()?,
			state: ListState { name },
		})
	}
}

#[async_trait]
impl Storage for SqliteStorage {
	async fn create_list(&self, state: ListState) -> Result<List, StorageError> {
		let id = Uuid::new_v4();
		let id_string = id.to_string();

		sqlx::query!(
			r#"
		    INSERT INTO lists (id, name)
		    VALUES (?, ?)
		    "#,
			id_string,
			state.name
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(List {
			id,
			items: Vec::new(),
			state: ListState {
				name: state.name.to_string(),
			},
		})
	}

	async fn get_list_overview(&self) -> Result<Vec<ListOverview>, StorageError> {
		let record = sqlx::query!(
			r#"
			SELECT id, name FROM lists
			"#
		)
		.fetch_all(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(record
			.iter()
			.map(|r| {
				Ok(ListOverview {
					id: Uuid::parse_str(&r.id)?,
					name: r.name.clone(),
				})
			})
			.collect::<Result<Vec<_>, StorageError>>()?)
	}

	async fn get_list(&self, list_id: Uuid) -> Result<ListOverview, StorageError> {
		let list_id_string = list_id.to_string();

		let list = sqlx::query!(
			r#"
			SELECT id, name FROM lists
			WHERE id = ?
			"#,
			list_id_string,
		)
		.fetch_one(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(ListOverview {
			id: Uuid::parse_str(&list.id)?,
			name: list.name,
		})
	}

	async fn update_list(&self, list_id: Uuid, state: ListState) -> Result<List, StorageError> {
		let list_id_string = list_id.to_string();

		let result = sqlx::query!(
			r#"
			UPDATE lists
			SET name = ?
			WHERE id = ?
			"#,
			state.name,
			list_id_string,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		if result.rows_affected() == 0 {
			return Err(StorageError::NotFound);
		}

		self.get_list(list_id).await
	}

	async fn delete_list(&self, list_id: Uuid) -> Result<(), StorageError> {
		let list_id_string = list_id.to_string();

		let result = sqlx::query!(
			r#"
	        DELETE FROM lists
	        WHERE id = ?
	        "#,
			list_id_string,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		if result.rows_affected() == 0 {
			return Err(StorageError::NotFound);
		}

		Ok(())
	}

	async fn create_task(&self, list_id: Uuid, state: TaskState) -> Result<Task, StorageError> {
		let task = Task {
			id: Uuid::new_v4(),
			state,
		};

		let task_id_string = task.id.to_string();
		let list_id_string = list_id.to_string();

		sqlx::query!(
			r#"
			INSERT INTO tasks (id, list_id, name, description, completed)
			VALUES (?, ?, ?, ?, ?)
			"#,
			task_id_string,
			list_id_string,
			task.state.name,
			task.state.description,
			task.state.completed,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(task)
	}

	async fn get_task_overview(&self, list_id: Uuid) -> Result<Vec<TaskOverview>, StorageError> {
		let list_id_string = list_id.to_string();

		let record = sqlx::query!(
			r#"
			SELECT id, name, completed FROM tasks
			WHERE list_id = ?
			"#,
			list_id_string,
		)
		.fetch_all(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(record
			.iter()
			.map(|r| {
				Ok(TaskOverview {
					id: Uuid::parse_str(&r.id)?,
					name: r.name.clone(),
					completed: r.completed,
				})
			})
			.collect::<Result<Vec<_>, StorageError>>()?)
	}

	async fn get_task(&self, task_id: Uuid) -> Result<Task, StorageError> {
		let task_id_string = task_id.to_string();

		let task = sqlx::query!(
			r#"
			SELECT * FROM tasks
			WHERE id = ?
			"#,
			task_id_string,
		)
		.fetch_one(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(Task {
			id: task_id,
			state: TaskState {
				name: task.name,
				description: task.description,
				completed: task.completed,
			},
		})
	}

	async fn update_task(&self, task_id: Uuid, state: TaskState) -> Result<Task, StorageError> {
		let task_id_string = task_id.to_string();

		let result = sqlx::query!(
			r#"
			UPDATE tasks
			SET (name, description, completed) = (?, ?, ?)
			WHERE id = ?
			"#,
			state.name,
			state.description,
			state.completed,
			task_id_string,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		if result.rows_affected() == 0 {
			return Err(StorageError::NotFound);
		}

		self.get_task(task_id).await
	}

	async fn delete_task(&self, task_id: Uuid) -> Result<(), StorageError> {
		let task_id_string = task_id.to_string();

		let result = sqlx::query!(
			r#"
	        DELETE FROM tasks
	        WHERE id = ?
	        "#,
			task_id_string,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		if result.rows_affected() == 0 {
			return Err(StorageError::NotFound);
		}

		Ok(())
	}
}
