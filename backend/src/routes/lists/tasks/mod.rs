use std::sync::Arc;

use axum::routing::{Router, delete, get, post, put};

use crate::storage::Storage;

pub mod task;

pub fn tasks() -> Router<Arc<dyn Storage>> {
	Router::new()
		.route("/", post(task::new))
		.route("/", get(task::get_tasks_overview))
		.route("/{task_id}", get(task::get))
		.route("/{task_id}", put(task::update))
		.route("/{task_id}", delete(task::delete))
		.route("/{task_id}/completed", put(task::set_completed))
		.route("/{task_id}/tags/{tag_id}/apply", post(task::apply_tag))
		.route("/{task_id}/tags/{tag_id}/remove", post(task::remove_tag))
}
