use std::sync::Arc;

use axum::routing::{Router, delete, get, post, put};

use crate::{authorization::ProtectedRoutes, permissions::ListPermission, storage::Storage};

pub mod task;

pub fn tasks() -> Router<Arc<dyn Storage>> {
	Router::new()
		.protected_route("/", post(task::new), ListPermission::AddTask)
		.protected_route("/", get(task::get_tasks_overview), ListPermission::Read)
		.protected_route("/{task_id}", get(task::get), ListPermission::Read)
		.protected_route("/{task_id}", put(task::update), ListPermission::Edit)
		.protected_route(
			"/{task_id}",
			delete(task::delete),
			ListPermission::DeleteTask,
		)
		.protected_route(
			"/{task_id}/completed",
			put(task::set_completed),
			ListPermission::Edit,
		)
		.protected_route(
			"/{task_id}/tags/{tag_id}/apply",
			post(task::apply_tag),
			ListPermission::Edit,
		)
		.protected_route(
			"/{task_id}/tags/{tag_id}/remove",
			post(task::remove_tag),
			ListPermission::Edit,
		)
}
