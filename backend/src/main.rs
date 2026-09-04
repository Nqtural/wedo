use axum::Router;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod routes;
mod storage;
mod types;

#[tokio::main]
async fn main() {
	dotenvy::dotenv().ok();

	let bind_address = std::env::var("BIND_ADDRESS").unwrap_or("0.0.0.0:3000".to_string());
	let frontend_url = std::env::var("FRONTEND_URL").unwrap_or("http://localhost:5173".to_string());
	let database_url = std::env::var("DATABASE_URL").unwrap_or("sqlite://app.db".to_string());

	let cors = CorsLayer::new()
		.allow_origin(frontend_url.parse::<axum::http::HeaderValue>().unwrap())
		.allow_methods(Any)
		.allow_headers(Any);

	let storage = match storage::sqlite::SqliteStorage::new(&database_url).await {
		Ok(storage) => storage,
		Err(_) => panic!("fatal: Failed to create storage instance"),
	};

	let app = Router::new()
		.merge(routes::routes().with_state(Arc::new(storage)))
		.layer(cors);

	let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
