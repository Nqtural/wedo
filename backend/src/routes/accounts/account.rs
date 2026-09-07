use crate::{
	storage::{Storage, StorageError},
	types::Credentials,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

pub async fn create(
	State(storage): State<Arc<dyn Storage>>,
	Json(credentials): Json<Credentials>,
) -> impl IntoResponse {
	if credentials.username.len() < 3 {
		return (
			StatusCode::BAD_REQUEST,
			Json("error: Username can not be shorter than 3 characters"),
		)
			.into_response();
	}

	match storage.create_account(&credentials).await {
		Ok(_) => StatusCode::CREATED.into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

fn decode_storage_error(error: StorageError) -> impl IntoResponse {
	match error {
		StorageError::NotFound => (StatusCode::NOT_FOUND, Json("error: Account not found")),
		StorageError::Conflict => (StatusCode::CONFLICT, Json("error: Username already exists")),
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
