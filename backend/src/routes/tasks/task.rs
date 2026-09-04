use crate::{
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

// pub async fn get_overview(
// 	State(storage): State<Arc<dyn Storage>>,
// 	Path(list_id): Path<Uuid>,
// ) -> impl IntoResponse {
// 	match storage.get_task_overview(list_id).await {
// 		Ok(lists) => (StatusCode::OK, Json(lists)).into_response(),
// 		Err(error) => decode_storage_error(error).into_response(),
// 	}
// }

pub async fn get(
	State(storage): State<Arc<dyn Storage>>,
	Path(task_id): Path<Uuid>,
) -> impl IntoResponse {
	match storage.get_task(task_id).await {
		Ok(list) => (StatusCode::OK, Json(list)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn update(
	State(storage): State<Arc<dyn Storage>>,
	Path(task_id): Path<Uuid>,
	Json(request): Json<TaskState>,
) -> impl IntoResponse {
	match storage.update_task(task_id, request).await {
		Ok(list) => (StatusCode::OK, Json(list)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn delete(
	State(storage): State<Arc<dyn Storage>>,
	Path(task_id): Path<Uuid>,
) -> impl IntoResponse {
	match storage.delete_task(task_id).await {
		Ok(_) => StatusCode::NO_CONTENT.into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

#[derive(Serialize, Deserialize)]
pub struct SetCompleted {
	completed: bool,
}

pub async fn set_completed(
	State(storage): State<Arc<dyn Storage>>,
	Path(task_id): Path<Uuid>,
	Json(request): Json<SetCompleted>,
) -> impl IntoResponse {
	let mut task = match storage.get_task(task_id).await {
		Ok(task) => task,
		Err(error) => return decode_storage_error(error).into_response(),
	};

	task.state.completed = request.completed;

	match storage.update_task(task_id, task.state).await {
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

fn decode_storage_error(error: StorageError) -> impl IntoResponse {
	match error {
		StorageError::NotFound => (StatusCode::NOT_FOUND, Json("error: List not found")),
		StorageError::Uuid(_) => unreachable!(),
		StorageError::Database(_) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			Json("error: Database error"),
		),
	}
}
