use crate::{
	authorization::Require,
	storage::{AuthError, Storage, StorageError},
	types::Credentials,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

pub async fn login(
	State(storage): State<Arc<dyn Storage>>,
	Json(request): Json<Credentials>,
) -> impl IntoResponse {
	match storage.create_session(&request).await {
		Ok(session_id) => (StatusCode::OK, Json(session_id)).into_response(),
		Err(error) => decode_auth_error(error).into_response(),
	}
}

pub async fn logout(
	require: Require,
	State(storage): State<Arc<dyn Storage>>,
) -> impl IntoResponse {
	let account_id = require.account_id();

	match storage.delete_session(account_id).await {
		Ok(()) => StatusCode::NO_CONTENT.into_response(),
		Err(error) => decode_storage_error(error).into_response(),
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
		StorageError::NotFound => (StatusCode::NOT_FOUND, Json("error: Session not found")),
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
