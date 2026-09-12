use std::sync::Arc;

use axum::routing::{Router, delete, get, post, put};

use crate::{authorization::ProtectedRoutes, permissions::ListPermission, storage::Storage};

pub mod tag;

pub fn tags() -> Router<Arc<dyn Storage>> {
	Router::new()
		.protected_route("/", get(tag::get), ListPermission::Read)
		.protected_route("/", post(tag::new), ListPermission::Edit)
		.protected_route("/{tag_id}", put(tag::update), ListPermission::Edit)
		.protected_route("/{tag_id}", delete(tag::delete), ListPermission::Edit)
}
