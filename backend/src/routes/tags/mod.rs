use std::sync::Arc;

use axum::routing::{Router, delete, put};

use crate::storage::Storage;

pub mod tag;

pub fn tags() -> Router<Arc<dyn Storage>> {
	Router::new()
		.route("/{tag_id}", put(tag::update))
		.route("/{tag_id}", delete(tag::delete))
}
