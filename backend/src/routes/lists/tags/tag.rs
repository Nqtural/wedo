use crate::{
	authorization::Require,
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

pub async fn get(require: Require, State(storage): State<Arc<dyn Storage>>) -> impl IntoResponse {
	let access = require.list();

	match storage
		.get_list_tags(access.account_id, access.list_id)
		.await
	{
		Ok(tags) => (StatusCode::OK, Json(tags)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn new(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Json(request): Json<TagState>,
) -> impl IntoResponse {
	let access = require.list();

	match storage
		.create_tag(access.account_id, access.list_id, request)
		.await
	{
		Ok(_) => StatusCode::CREATED.into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn update(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Path((_list_id, tag_id)): Path<(Uuid, Uuid)>,
	Json(request): Json<TagState>,
) -> impl IntoResponse {
	let access = require.list();

	match storage.update_tag(access.account_id, tag_id, request).await {
		Ok(tag) => (StatusCode::OK, Json(tag)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn delete(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Path((_list_id, tag_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
	let access = require.list();

	match storage.delete_tag(access.account_id, tag_id).await {
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
