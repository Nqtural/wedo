use std::sync::Arc;

use axum::Router;

use crate::storage::Storage;

pub mod accounts;
pub mod lists;
pub mod tasks;

pub fn routes() -> Router<Arc<dyn Storage>> {
	Router::new()
		.nest("/accounts", accounts::accounts())
		.nest("/lists", lists::lists())
		.nest("/tasks", tasks::tasks())
}
