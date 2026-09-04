use axum::Router;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod routes;
mod storage;
mod types;

#[allow(unused)] // linter runs with debug assertions
enum Mode {
	Development,
	Production,
}

#[tokio::main]
async fn main() {
	#[cfg(not(debug_assertions))]
	let mode = Mode::Production;
	#[cfg(debug_assertions)]
	let mode = Mode::Development;

	let (bind_address, frontend_url, database_url) = if matches!(mode, Mode::Development) {
		println!("info: Running in development mode");
		(
			"0.0.0.0:3000".to_string(),
			"http://localhost:5173".to_string(),
			"sqlite://app.db".to_string(),
		)
	} else {
		dotenvy::dotenv().ok();
		(
			std::env::var("BIND_ADDRESS")
				.expect("fatal: BIND_ADDRESS environment variable not set"),
			std::env::var("FRONTEND_URL")
				.expect("fatal: FRONTEND_URL environment variable not set"),
			std::env::var("DATABASE_URL")
				.expect("fatal: DATABASE_URL environment variable not set"),
		)
	};

	// not needed if backend and frontend are hosted on
	// the same origin, kept for people that may want it
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
