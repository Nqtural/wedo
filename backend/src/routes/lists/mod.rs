use std::sync::Arc;

use axum::routing::{Router, delete, get, post, put};

use crate::storage::Storage;

pub mod list;
pub mod tags;
pub mod tasks;

pub fn lists() -> Router<Arc<dyn Storage>> {
	Router::new()
		.route("/", post(list::new))
		.route("/", get(list::get_overview))
		.route("/join/{invitation_id}", post(list::join))
		.route("/{list_id}", get(list::get))
		.route("/{list_id}", put(list::rename))
		.route("/{list_id}", delete(list::delete))
		.route("/{list_id}/share", post(list::share))
		.nest("/{listId}/tags", tags::tags())
		.nest("/{listId}/tasks", tasks::tasks())
}
