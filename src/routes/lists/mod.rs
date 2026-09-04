use axum::{
	Router,
	routing::{delete, get, post, put},
};

pub mod list;
pub mod todo;

pub fn lists() -> Router {
	Router::new()
		.route("/", post(list::new))
		.route("/{list_id}", get(list::get))
		.route("/{list_id}", put(list::update))
		.route("/{list_id}", delete(list::delete))
		.route("/{list_id}/todos", post(todo::new))
		.route("/{list_id}/todos", get(todo::get_list))
		.route("/{list_id}/todos/{todo_id}", get(todo::get_item))
		.route("/{list_id}/todos/{todo_id}", put(todo::update))
		.route("/{list_id}/todos/{todo_id}", delete(todo::delete))
}
