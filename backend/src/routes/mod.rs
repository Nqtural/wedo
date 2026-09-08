use std::sync::Arc;

use axum::Router;

use crate::storage::Storage;

pub mod accounts;
pub mod auth;
pub mod lists;
pub mod tags;
pub mod tasks;

pub fn routes() -> Router<Arc<dyn Storage>> {
	Router::new()
		.nest("/accounts", accounts::accounts())
		.nest("/auth", auth::auth())
		.nest("/lists", lists::lists())
		.nest("/tags", tags::tags())
		.nest("/tasks", tasks::tasks())
}
