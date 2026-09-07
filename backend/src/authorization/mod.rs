use axum::extract::FromRequestParts;
use http::{StatusCode, request::Parts};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
	pub account_id: Uuid,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
	S: Send + Sync,
{
	type Rejection = StatusCode;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
		let value = parts
			.headers
			.get("Authorization")
			.ok_or(StatusCode::UNAUTHORIZED)?;

		let value = value.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

		let account_id = Uuid::parse_str(value).map_err(|_| StatusCode::UNAUTHORIZED)?;

		Ok(Self { account_id })
	}
}
