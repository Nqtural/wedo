use std::sync::Arc;

use axum::routing::{Router, delete, get, post, put};

use crate::storage::Storage;

pub mod list;

pub fn lists() -> Router<Arc<dyn Storage>> {
	Router::new()
		.route("/", post(list::new))
		.route("/", get(list::get_overview))
		.route("/{list_id}", put(list::rename))
		.route("/{list_id}", delete(list::delete))
		.route("/{list_id}/tasks", post(list::new_task))
		.route("/{list_id}/tasks", get(list::get_task_overview))
}
