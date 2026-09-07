use std::sync::Arc;

use axum::routing::{Router, post};

use crate::storage::Storage;

pub mod account;

pub fn accounts() -> Router<Arc<dyn Storage>> {
	Router::new().route("/", post(account::create))
}
