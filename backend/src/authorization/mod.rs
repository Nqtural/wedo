use axum::{Extension, Router, extract::FromRequestParts, routing::MethodRouter};
use http::{StatusCode, request::Parts};
use std::sync::Arc;
use uuid::Uuid;

use crate::{permissions::ListPermission, storage::Storage};

#[derive(Debug, Clone)]
pub enum Requirement {
	Authenticated,
	List(ListPermission),
}

#[derive(Debug, Clone)]
pub enum Require {
	Authenticated { account_id: Uuid },
	List(ListAccess),
}

impl Require {
	pub fn account_id(&self) -> Uuid {
		match self {
			Require::Authenticated { account_id } => *account_id,
			Require::List(access) => access.account_id,
		}
	}

	pub fn list(&self) -> &ListAccess {
		match self {
			Require::List(access) => access,
			Require::Authenticated { .. } => {
				unreachable!("route requires list access")
			}
		}
	}
}

impl FromRequestParts<Arc<dyn Storage>> for Require {
	type Rejection = StatusCode;

	async fn from_request_parts(
		parts: &mut Parts,
		storage: &Arc<dyn Storage>,
	) -> Result<Self, Self::Rejection> {
		let requirement = parts
			.extensions
			.get::<Requirement>()
			.cloned()
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

		let account_id = authenticate(parts, storage).await?;

		match requirement {
			Requirement::Authenticated => Ok(Require::Authenticated { account_id }),

			Requirement::List(permission) => {
				let params = axum::extract::RawPathParams::from_request_parts(parts, storage)
					.await
					.map_err(|_| StatusCode::BAD_REQUEST)?;

				let list_id = params
					.iter()
					.find(|(key, _)| *key == "list_id")
					.and_then(|(_, value)| Uuid::parse_str(value).ok())
					.ok_or(StatusCode::BAD_REQUEST)?;

				let role = storage
					.get_list_role(account_id, list_id)
					.await
					.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
					.ok_or(StatusCode::FORBIDDEN)?;

				if !role.can(permission) {
					return Err(StatusCode::FORBIDDEN);
				}

				Ok(Self::List(ListAccess {
					account_id,
					list_id,
				}))
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct ListAccess {
	pub account_id: Uuid,
	pub list_id: Uuid,
}

async fn authenticate(parts: &mut Parts, storage: &Arc<dyn Storage>) -> Result<Uuid, StatusCode> {
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

	Ok(account_id)
}

pub trait ProtectedRoutes {
	fn authenticated_route(self, path: &str, method_router: MethodRouter<Arc<dyn Storage>>)
	-> Self;

	fn protected_route(
		self,
		path: &str,
		method_router: MethodRouter<Arc<dyn Storage>>,
		permission: ListPermission,
	) -> Self;
}

impl ProtectedRoutes for Router<Arc<dyn Storage>> {
	fn authenticated_route(
		self,
		path: &str,
		method_router: MethodRouter<Arc<dyn Storage>>,
	) -> Self {
		self.route(
			path,
			method_router.layer(Extension(Requirement::Authenticated)),
		)
	}

	fn protected_route(
		self,
		path: &str,
		method_router: MethodRouter<Arc<dyn Storage>>,
		permission: ListPermission,
	) -> Self {
		self.route(
			path,
			method_router.layer(Extension(Requirement::List(permission))),
		)
	}
}
