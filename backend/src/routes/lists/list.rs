use crate::{
	authorization::Require,
	storage::{AuthError, Storage, StorageError},
	types::ListState,
};
use axum::{
	Json,
	extract::{Path, State},
	http::StatusCode,
	response::IntoResponse,
};
use std::sync::Arc;

pub async fn new(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Json(request): Json<ListState>,
) -> impl IntoResponse {
	let account_id = require.account_id();

	match storage.create_list(account_id, request).await {
		Ok(list) => (StatusCode::CREATED, Json(list)).into_response(),
		Err(_) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			Json("error: Failed to create list"),
		)
			.into_response(),
	}
}

pub async fn get_overview(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
) -> impl IntoResponse {
	let account_id = require.account_id();

	match storage.get_list_overview(account_id).await {
		Ok(lists) => (StatusCode::OK, Json(lists)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn join(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Path(invitation_id): Path<String>,
) -> impl IntoResponse {
	let account_id = require.account_id();

	match storage.accept_invitation(account_id, invitation_id).await {
		Ok(join_result) => (StatusCode::OK, Json(join_result)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn get(require: Require, State(storage): State<Arc<dyn Storage>>) -> impl IntoResponse {
	let access = require.list();

	match storage.get_list(access.account_id, access.list_id).await {
		Ok(list) => (StatusCode::OK, Json(list)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn rename(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
	Json(request): Json<ListState>,
) -> impl IntoResponse {
	let access = require.list();

	match storage
		.update_list(access.account_id, access.list_id, request)
		.await
	{
		Ok(list) => (StatusCode::OK, Json(list)).into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn delete(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
) -> impl IntoResponse {
	let access = require.list();

	match storage.delete_list(access.account_id, access.list_id).await {
		Ok(_) => StatusCode::NO_CONTENT.into_response(),
		Err(error) => decode_storage_error(error).into_response(),
	}
}

pub async fn share(require: Require, State(storage): State<Arc<dyn Storage>>) -> impl IntoResponse {
	let access = require.list();

	match storage
		.create_invitation(access.account_id, access.list_id)
		.await
	{
		Ok(invitation_id) => (StatusCode::OK, Json(invitation_id)).into_response(),
		Err(error) => decode_auth_error(error).into_response(),
	}
}

fn decode_auth_error(error: AuthError) -> impl IntoResponse {
	match error {
		AuthError::Forbidden => StatusCode::FORBIDDEN.into_response(),
		AuthError::InvalidCredentials => {
			(StatusCode::UNAUTHORIZED, Json("error: Invalid credentials")).into_response()
		}
		AuthError::Storage(error) => decode_storage_error(error).into_response(),
		AuthError::Hash(_) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			Json("error: Failed to hash password"),
		)
			.into_response(),
	}
}

fn decode_storage_error(error: StorageError) -> impl IntoResponse {
	match error {
		StorageError::NotFound => (StatusCode::NOT_FOUND, Json("error: List not found")),
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
