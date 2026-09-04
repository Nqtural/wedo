use axum::Router;
use std::sync::Arc;

mod routes;
mod storage;
mod types;

#[tokio::main]
async fn main() {
	let storage = match storage::sqlite::SqliteStorage::new("sqlite://app.db").await {
		Ok(storage) => storage,
		Err(_) => panic!("fatal: Failed to create storage instance"),
	};

	let app = Router::new().merge(routes::routes().with_state(Arc::new(storage)));

	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
