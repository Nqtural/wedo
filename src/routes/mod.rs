use axum::Router;

pub mod lists;

pub fn routes() -> Router<()> {
	Router::new().nest("/lists", lists::lists())
}
