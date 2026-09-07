use axum::extract::FromRequestParts;
use http::{StatusCode, request::Parts};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::storage::Storage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
	pub account_id: Uuid,
}

impl FromRequestParts<Arc<dyn Storage>> for AuthenticatedUser {
	type Rejection = StatusCode;

	async fn from_request_parts(
		parts: &mut Parts,
		storage: &Arc<dyn Storage>,
	) -> Result<Self, Self::Rejection> {
		let value = parts
			.headers
			.get("Authorization")
			.ok_or(StatusCode::UNAUTHORIZED)?;
		let value = value.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;
		let session_id = Uuid::parse_str(value).map_err(|_| StatusCode::UNAUTHORIZED)?;
		let account_id = storage
			.validate_session(session_id)
			.await
			.map_err(|_| StatusCode::UNAUTHORIZED)?;

		Ok(Self { account_id })
	}
}
