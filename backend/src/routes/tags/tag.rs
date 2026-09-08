use crate::{
	authorization::AuthenticatedUser,
	storage::{Storage, StorageError},
	types::TagState,
};
use axum::{
	Json,
	extract::{Path, State},
	http::StatusCode,
	response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn update(
	AuthenticatedUser { account_id }: AuthenticatedUser,
	State(storage): State<Arc<dyn Storage>>,
	Path(tag_id): Path<Uuid>,
	Json(request): Json<TagState>,
) -> impl IntoResponse {
	match storage.update_tag(account_id, tag_id, request).await {
		Ok(tag) => (StatusCode::OK, Json(tag)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn delete(
	AuthenticatedUser { account_id }: AuthenticatedUser,
	State(storage): State<Arc<dyn Storage>>,
	Path(tag_id): Path<Uuid>,
) -> impl IntoResponse {
	match storage.delete_tag(account_id, tag_id).await {
		Ok(_) => StatusCode::NO_CONTENT.into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

fn decode_storage_error(error: StorageError) -> impl IntoResponse {
	match error {
		StorageError::NotFound => (StatusCode::NOT_FOUND, Json("error: Task not found")),
		StorageError::Conflict => (StatusCode::CONFLICT, Json("error: Conflict")),
		StorageError::Uuid(_) => unreachable!(),
		StorageError::Database(_) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			Json("error: Database error"),
		),
		StorageError::Hash(_) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			Json("error: Failed to hash password"),
		),
	}
}
