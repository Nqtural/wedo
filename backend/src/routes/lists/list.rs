use crate::{
	storage::{Storage, StorageError},
	types::{ListState, TaskState},
};
use axum::{
	Json,
	extract::{Path, State},
	http::StatusCode,
	response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn new(
	State(storage): State<Arc<dyn Storage>>,
	Json(request): Json<ListState>,
) -> impl IntoResponse {
	match storage.create_list(request).await {
		Ok(list) => (StatusCode::CREATED, Json(list)).into_response(),
		Err(_) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			Json("error: Failed to create list"),
		)
			.into_response(),
	}
}

pub async fn get_overview(State(storage): State<Arc<dyn Storage>>) -> impl IntoResponse {
	match storage.get_list_overview().await {
		Ok(lists) => (StatusCode::OK, Json(lists)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn rename(
	State(storage): State<Arc<dyn Storage>>,
	Path(list_id): Path<Uuid>,
	Json(request): Json<ListState>,
) -> impl IntoResponse {
	match storage.update_list(list_id, request).await {
		Ok(list) => (StatusCode::OK, Json(list)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn delete(
	State(storage): State<Arc<dyn Storage>>,
	Path(list_id): Path<Uuid>,
) -> impl IntoResponse {
	match storage.delete_list(list_id).await {
		Ok(_) => StatusCode::NO_CONTENT.into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn new_task(
	State(storage): State<Arc<dyn Storage>>,
	Path(list_id): Path<Uuid>,
	Json(request): Json<TaskState>,
) -> impl IntoResponse {
	match storage.create_task(list_id, request).await {
		Ok(task) => (StatusCode::CREATED, Json(task)).into_response(),
		Err(_) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			Json("error: Failed to create task"),
		)
			.into_response(),
	}
}

pub async fn get_task_overview(
	State(storage): State<Arc<dyn Storage>>,
	Path(list_id): Path<Uuid>,
) -> impl IntoResponse {
	match storage.get_task_overview(list_id).await {
		Ok(lists) => (StatusCode::OK, Json(lists)).into_response(),
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
