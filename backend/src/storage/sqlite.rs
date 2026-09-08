use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use rand::RngExt;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use uuid::Uuid;

use crate::storage::{AuthError, Storage, StorageError};
use crate::types::{
	Account, Credentials, JoinResult, List, ListOverview, ListState, Task, TaskOverview, TaskState,
};

const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn base62_id() -> String {
	let mut rng = rand::rng();

	(0..6)
		.map(|_| CHARSET[rng.random_range(0..62)] as char)
		.collect()
}

pub struct SqliteStorage {
	pool: SqlitePool,
}

impl SqliteStorage {
	pub async fn new(path: &str) -> Result<Self, sqlx::Error> {
		let pool = SqlitePoolOptions::new()
			.max_connections(5)
			.after_connect(|conn, _meta| {
				Box::pin(async move {
					sqlx::query("PRAGMA foreign_keys = ON")
						.execute(conn)
						.await?;

					Ok(())
				})
			})
			.connect(path)
			.await?;

		Ok(Self { pool })
	}

	async fn get_full_list(&self, account_id: Uuid, list_id: Uuid) -> Result<List, StorageError> {
		let list_id_string = list_id.to_string();
		let account_id_string = account_id.to_string();

		let list = sqlx::query!(
			r#"
			SELECT l.id, l.name
			FROM lists l
			INNER JOIN list_membership lm ON lm.list_id = l.id
			WHERE l.id = ?
			AND lm.user_id = ?
			"#,
			list_id_string,
			account_id_string,
		)
		.fetch_optional(&self.pool)
		.await
		.map_err(StorageError::Database)?
		.ok_or(StorageError::NotFound)?;

		let tasks = sqlx::query!(
			r#"
			SELECT id, name, description, completed
			FROM tasks
			WHERE list_id = ?
			"#,
			list_id_string,
		)
		.fetch_all(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(List {
			id: Uuid::parse_str(&list.id)?,
			state: ListState { name: list.name },
			items: tasks
				.into_iter()
				.map(|r| {
					Ok(Task {
						id: Uuid::parse_str(&r.id)?,
						state: TaskState {
							name: r.name,
							description: r.description,
							completed: r.completed,
						},
					})
				})
				.collect::<Result<Vec<_>, StorageError>>()?,
		})
	}
}

#[async_trait]
impl Storage for SqliteStorage {
	async fn create_list(&self, account_id: Uuid, state: ListState) -> Result<List, StorageError> {
		let list_id = Uuid::new_v4();
		let list_id_string = list_id.to_string();
		let account_id_string = account_id.to_string();
		let role = "OWNER".to_string();

		let mut tx = self.pool.begin().await.map_err(StorageError::Database)?;

		sqlx::query!(
			r#"
			INSERT INTO lists (id, name)
			VALUES (?, ?)
			"#,
			list_id_string,
			state.name
		)
		.execute(&mut *tx)
		.await
		.map_err(StorageError::Database)?;

		sqlx::query!(
			r#"
			INSERT INTO list_membership (list_id, user_id, role)
			VALUES (?, ?, ?)
			"#,
			list_id_string,
			account_id_string,
			role,
		)
		.execute(&mut *tx)
		.await
		.map_err(StorageError::Database)?;

		tx.commit().await.map_err(StorageError::Database)?;

		Ok(List {
			id: list_id,
			items: Vec::new(),
			state: ListState {
				name: state.name.to_string(),
			},
		})
	}

	async fn get_list_overview(&self, account_id: Uuid) -> Result<Vec<ListOverview>, StorageError> {
		let account_id_string = account_id.to_string();

		let record = sqlx::query!(
			r#"
			SELECT l.id, l.name
			FROM lists l
			INNER JOIN list_membership lm
			ON lm.list_id = l.id
			WHERE lm.user_id = ?
	        "#,
			account_id_string,
		)
		.fetch_all(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(record
			.into_iter()
			.map(|r| {
				Ok(ListOverview {
					id: Uuid::parse_str(&r.id)?,
					name: r.name,
				})
			})
			.collect::<Result<Vec<_>, StorageError>>()?)
	}

	async fn get_list(
		&self,
		account_id: Uuid,
		list_id: Uuid,
	) -> Result<ListOverview, StorageError> {
		let account_id_string = account_id.to_string();
		let list_id_string = list_id.to_string();

		let list = sqlx::query!(
			r#"
			SELECT id, name FROM lists
			WHERE id = ?
			AND EXISTS (
				SELECT 1
				FROM list_membership
				WHERE list_id = ?
				AND user_id = ?
			)
			"#,
			list_id_string,
			list_id_string,
			account_id_string,
		)
		.fetch_one(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(ListOverview {
			id: Uuid::parse_str(&list.id)?,
			name: list.name,
		})
	}

	async fn update_list(
		&self,
		account_id: Uuid,
		list_id: Uuid,
		state: ListState,
	) -> Result<List, StorageError> {
		let account_id_string = account_id.to_string();
		let list_id_string = list_id.to_string();

		let result = sqlx::query!(
			r#"
			UPDATE lists
			SET name = ?
			WHERE id = ?
			AND EXISTS (
				SELECT 1
				FROM list_membership
				WHERE list_id = ?
				AND user_id = ?
			)
		    "#,
			state.name,
			list_id_string,
			list_id_string,
			account_id_string,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		if result.rows_affected() == 0 {
			return Err(StorageError::NotFound);
		}

		self.get_full_list(account_id, list_id).await
	}

	async fn delete_list(&self, account_id: Uuid, list_id: Uuid) -> Result<(), StorageError> {
		let account_id_string = account_id.to_string();
		let list_id_string = list_id.to_string();

		let result = sqlx::query!(
			r#"
			DELETE FROM lists
			WHERE id = ?
			AND EXISTS (
				SELECT 1
				FROM list_membership
				WHERE list_id = ?
				AND user_id = ?
			)
			"#,
			list_id_string,
			list_id_string,
			account_id_string,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		if result.rows_affected() == 0 {
			return Err(StorageError::NotFound);
		}

		Ok(())
	}

	async fn create_task(
		&self,
		account_id: Uuid,
		list_id: Uuid,
		state: TaskState,
	) -> Result<Task, StorageError> {
		let account_id_string = account_id.to_string();
		let task_id = Uuid::new_v4();
		let task_id_string = task_id.to_string();
		let list_id_string = list_id.to_string();

		let task = Task { id: task_id, state };

		let result = sqlx::query!(
			r#"
			INSERT INTO tasks (id, list_id, name, description, completed)
			SELECT ?, ?, ?, ?, ?
			WHERE EXISTS (
				SELECT 1
				FROM list_membership
				WHERE list_id = ?
				AND user_id = ?
			)
			"#,
			task_id_string,
			list_id_string,
			task.state.name,
			task.state.description,
			task.state.completed,
			list_id_string,
			account_id_string,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		if result.rows_affected() == 0 {
			return Err(StorageError::NotFound);
		}

		Ok(task)
	}

	async fn get_task_overview(
		&self,
		account_id: Uuid,
		list_id: Uuid,
	) -> Result<Vec<TaskOverview>, StorageError> {
		let account_id_string = account_id.to_string();
		let list_id_string = list_id.to_string();

		let record = sqlx::query!(
			r#"
			SELECT t.id, t.name, t.completed
			FROM tasks t
			WHERE t.list_id = ?
			AND EXISTS (
				SELECT 1
				FROM list_membership lm
				WHERE lm.list_id = t.list_id
				AND lm.user_id = ?
			)
			"#,
			list_id_string,
			account_id_string,
		)
		.fetch_all(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(record
			.into_iter()
			.map(|r| {
				Ok(TaskOverview {
					id: Uuid::parse_str(&r.id)?,
					name: r.name,
					completed: r.completed,
				})
			})
			.collect::<Result<Vec<_>, StorageError>>()?)
	}

	async fn get_task(&self, account_id: Uuid, task_id: Uuid) -> Result<Task, StorageError> {
		let task_id_string = task_id.to_string();
		let account_id_string = account_id.to_string();

		let task = sqlx::query!(
			r#"
			SELECT t.id, t.name, t.description, t.completed
			FROM tasks t
			INNER JOIN list_membership lm
			ON lm.list_id = t.list_id
			WHERE t.id = ?
			AND lm.user_id = ?
			"#,
			task_id_string,
			account_id_string,
		)
		.fetch_optional(&self.pool)
		.await
		.map_err(StorageError::Database)?
		.ok_or(StorageError::NotFound)?;

		Ok(Task {
			id: Uuid::parse_str(&task.id)?,
			state: TaskState {
				name: task.name,
				description: task.description,
				completed: task.completed,
			},
		})
	}

	async fn update_task(
		&self,
		account_id: Uuid,
		task_id: Uuid,
		state: TaskState,
	) -> Result<Task, StorageError> {
		let account_id_string = account_id.to_string();
		let task_id_string = task_id.to_string();

		let result = sqlx::query!(
			r#"
			UPDATE tasks
			SET name = ?, description = ?, completed = ?
			WHERE id = ?
			AND EXISTS (
				SELECT 1
				FROM list_membership lm
				WHERE lm.list_id = tasks.list_id
				AND lm.user_id = ?
			)
			"#,
			state.name,
			state.description,
			state.completed,
			task_id_string,
			account_id_string,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		if result.rows_affected() == 0 {
			return Err(StorageError::NotFound);
		}

		self.get_task(account_id, task_id).await
	}

	async fn delete_task(&self, account_id: Uuid, task_id: Uuid) -> Result<(), StorageError> {
		let account_id_string = account_id.to_string();
		let task_id_string = task_id.to_string();

		let result = sqlx::query!(
			r#"
			DELETE FROM tasks
			WHERE id = ?
			AND EXISTS (
				SELECT 1
				FROM list_membership lm
				WHERE lm.list_id = tasks.list_id
				AND lm.user_id = ?
			)
			"#,
			task_id_string,
			account_id_string,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		if result.rows_affected() == 0 {
			return Err(StorageError::NotFound);
		}

		Ok(())
	}

	async fn create_account(&self, credentials: &Credentials) -> Result<(), StorageError> {
		let password_hash =
			hash(&credentials.password, DEFAULT_COST).map_err(StorageError::Hash)?;
		let account_id_string = Uuid::new_v4().to_string();

		sqlx::query!(
			r#"
			INSERT INTO accounts (id, username, password_hash)
			VALUES (?, ?, ?)
			"#,
			account_id_string,
			credentials.username,
			password_hash,
		)
		.execute(&self.pool)
		.await
		.map_err(|err| {
			if let sqlx::Error::Database(db_err) = &err {
				if db_err.is_unique_violation() {
					return StorageError::Conflict;
				}
			}

			StorageError::Database(err)
		})?;

		Ok(())
	}

	async fn get_account(&self, account_id: Uuid) -> Result<Account, StorageError> {
		let account_id_string = account_id.to_string();

		let record = sqlx::query!(
			r#"
			SELECT id, username, created_at FROM accounts
			WHERE id = ?
			"#,
			account_id_string,
		)
		.fetch_one(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(Account {
			id: Uuid::parse_str(&record.id)?,
			username: record.username,
			created_at: record.created_at,
		})
	}

	async fn delete_account(&self, account_id: Uuid) -> Result<(), StorageError> {
		let account_id_string = account_id.to_string();

		let result = sqlx::query!(
			r#"
	        DELETE FROM accounts
	        WHERE id = ?
	        "#,
			account_id_string,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		if result.rows_affected() == 0 {
			return Err(StorageError::NotFound);
		}

		Ok(())
	}

	async fn create_session(&self, credentials: &Credentials) -> Result<Uuid, AuthError> {
		let account = sqlx::query!(
			r#"
			SELECT id, password_hash
			FROM accounts
			WHERE username = ?
			"#,
			credentials.username,
		)
		.fetch_optional(&self.pool)
		.await
		.map_err(StorageError::Database)?
		.ok_or(AuthError::InvalidCredentials)?;

		if !verify(&credentials.password, &account.password_hash)? {
			return Err(AuthError::InvalidCredentials);
		}

		let session_id = Uuid::new_v4();
		let session_id_string = session_id.to_string();
		let account_id_string = account.id.to_string();
		let expires_at = (Utc::now() + Duration::days(7)).naive_utc();

		sqlx::query!(
			r#"
			INSERT INTO sessions (id, account_id, expires_at)
			VALUES (?, ?, ?)
			"#,
			session_id_string,
			account_id_string,
			expires_at,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(session_id)
	}

	async fn validate_session(&self, session_id: Uuid) -> Result<Uuid, StorageError> {
		let session_id_string = session_id.to_string();
		let mut tx = self.pool.begin().await.map_err(StorageError::Database)?;
		let now = Utc::now().naive_utc();

		sqlx::query!(
			r#"
			DELETE FROM sessions
			WHERE expires_at <= ?
			"#,
			now,
		)
		.execute(&mut *tx)
		.await
		.map_err(StorageError::Database)?;

		let account_id = sqlx::query!(
			r#"
			SELECT account_id
			FROM sessions
			WHERE id = ?
			"#,
			session_id_string,
		)
		.fetch_optional(&mut *tx)
		.await
		.map_err(StorageError::Database)?
		.ok_or(StorageError::NotFound)?
		.account_id;

		tx.commit().await.map_err(StorageError::Database)?;

		Ok(Uuid::parse_str(&account_id)?)
	}

	async fn delete_session(&self, session_id: Uuid) -> Result<(), StorageError> {
		let session_id_string = session_id.to_string();

		sqlx::query!(
			r#"
	        DELETE FROM sessions
	        WHERE id = ?
	        "#,
			session_id_string,
		)
		.execute(&self.pool)
		.await
		.map_err(StorageError::Database)?;

		Ok(())
	}

	async fn create_invitation(
		&self,
		account_id: Uuid,
		list_id: Uuid,
	) -> Result<String, AuthError> {
		let account_id_string = account_id.to_string();
		let list_id_string = list_id.to_string();

		const MAX_ATTEMPTS: usize = 5;

		for _ in 0..MAX_ATTEMPTS {
			let invitation_id = base62_id();
			let expires_at = (Utc::now() + Duration::days(1)).naive_utc();

			let result = sqlx::query!(
				r#"
				INSERT INTO invitations (id, account_id, list_id, expires_at)
				SELECT ?, ?, ?, ?
				WHERE EXISTS (
					SELECT 1
					FROM list_membership
					WHERE list_id = ?
					AND user_id = ?
				)
				"#,
				invitation_id,
				account_id_string,
				list_id_string,
				expires_at,
				list_id_string,
				account_id_string,
			)
			.execute(&self.pool)
			.await;

			match result {
				Ok(result) if result.rows_affected() == 1 => {
					return Ok(invitation_id);
				}
				Ok(_) => {
					// not a member of the list
					return Err(AuthError::Forbidden);
				}
				Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
					continue;
				}
				Err(err) => return Err(AuthError::Storage(StorageError::Database(err))),
			}
		}

		Err(AuthError::Storage(StorageError::Database(
			sqlx::Error::Protocol("failed to generate a unique invitation ID".into()),
		)))
	}

	async fn accept_invitation(
		&self,
		account_id: Uuid,
		invitation_id: String,
	) -> Result<JoinResult, StorageError> {
		let account_id_string = account_id.to_string();
		let role = "OWNER".to_string(); // everyone is owner for now
		let now = Utc::now().naive_utc();

		let mut tx = self.pool.begin().await.map_err(StorageError::Database)?;

		let list_id_string = sqlx::query!(
			r#"
			SELECT list_id
			FROM invitations
			WHERE id = ?
			AND expires_at > ?
			"#,
			invitation_id,
			now,
		)
		.fetch_optional(&mut *tx)
		.await
		.map_err(StorageError::Database)?
		.ok_or(StorageError::NotFound)?
		.list_id;

		let joined = match sqlx::query!(
			r#"
			INSERT INTO list_membership (list_id, user_id, role)
			VALUES (?, ?, ?)
			ON CONFLICT (list_id, user_id) DO NOTHING
			"#,
			list_id_string,
			account_id_string,
			role,
		)
		.execute(&mut *tx)
		.await
		.map_err(StorageError::Database)?
		.rows_affected()
		{
			1 => true,
			0 => false,
			_ => unreachable!(),
		};

		let record = sqlx::query!(
			r#"
			SELECT id, name
			FROM lists
			WHERE id = ?
			"#,
			list_id_string,
		)
		.fetch_one(&mut *tx)
		.await
		.map_err(StorageError::Database)?;

		tx.commit().await.map_err(StorageError::Database)?;

		Ok(JoinResult {
			list: ListOverview {
				id: Uuid::parse_str(&record.id)?,
				name: record.name,
			},
			joined,
		})
	}
}
