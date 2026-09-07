use std::sync::Arc;

use axum::routing::{Router, delete, get, post, put};

use crate::storage::Storage;

pub mod authentication;

pub fn auth() -> Router<Arc<dyn Storage>> {
	Router::new()
		.route("/login", post(authentication::login))
		.route("/logout", post(authentication::logout))
}
