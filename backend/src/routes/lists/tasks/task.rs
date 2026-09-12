use crate::{
	authorization::Require,
	storage::{Storage, StorageError},
	types::TaskState,
};
use axum::{
	Json,
	extract::{Path, State},
	http::StatusCode,
	response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub async fn new(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Json(request): Json<TaskState>,
) -> impl IntoResponse {
	let access = require.list();

	match storage
		.create_task(access.account_id, access.list_id, request)
		.await
	{
		Ok(task) => (StatusCode::CREATED, Json(task)).into_response(),
		Err(_) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			Json("error: Failed to create task"),
		)
			.into_response(),
	}
}

pub async fn get_tasks_overview(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
) -> impl IntoResponse {
	let access = require.list();

	match storage
		.get_task_overview(access.account_id, access.list_id)
		.await
	{
		Ok(lists) => (StatusCode::OK, Json(lists)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn get(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Path((_list_id, task_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
	let access = require.list();

	match storage.get_task(access.account_id, task_id).await {
		Ok(list) => (StatusCode::OK, Json(list)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn update(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Path((_list_id, task_id)): Path<(Uuid, Uuid)>,
	Json(request): Json<TaskState>,
) -> impl IntoResponse {
	let access = require.list();

	match storage
		.update_task(access.account_id, task_id, request)
		.await
	{
		Ok(list) => (StatusCode::OK, Json(list)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn delete(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Path((_list_id, task_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
	let access = require.list();

	match storage.delete_task(access.account_id, task_id).await {
		Ok(_) => StatusCode::NO_CONTENT.into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

#[derive(Serialize, Deserialize)]
pub struct SetCompleted {
	completed: bool,
}

pub async fn set_completed(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Path((_list_id, task_id)): Path<(Uuid, Uuid)>,
	Json(request): Json<SetCompleted>,
) -> impl IntoResponse {
	let access = require.list();

	let mut task = match storage.get_task(access.account_id, task_id).await {
		Ok(task) => task,
		Err(error) => return decode_storage_error(error).into_response(),
	};

	task.state.completed = request.completed;

	match storage
		.update_task(access.account_id, task_id, task.state)
		.await
	{
		Ok(task) => (
			StatusCode::OK,
			Json(SetCompleted {
				completed: task.state.completed,
			}),
		)
			.into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn apply_tag(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Path((_list_id, task_id, tag_id)): Path<(Uuid, Uuid, Uuid)>,
) -> impl IntoResponse {
	let access = require.list();

	match storage.apply_tag(access.account_id, tag_id, task_id).await {
		Ok(tag) => (StatusCode::OK, Json(tag)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn remove_tag(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Path((_list_id, task_id, tag_id)): Path<(Uuid, Uuid, Uuid)>,
) -> impl IntoResponse {
	let access = require.list();

	match storage.remove_tag(access.account_id, tag_id, task_id).await {
		Ok(()) => StatusCode::OK.into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

fn decode_storage_error(error: StorageError) -> impl IntoResponse {
	match error {
		StorageError::NotFound => StatusCode::NOT_FOUND,
		StorageError::Conflict => StatusCode::CONFLICT,
		StorageError::Uuid(_) => unreachable!(),
		StorageError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
		StorageError::Hash(_) => StatusCode::INTERNAL_SERVER_ERROR,
	}
}
