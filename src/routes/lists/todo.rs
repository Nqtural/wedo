use axum::extract::Path;

pub async fn new(Path(list_id): Path<String>) {
	println!("Creating new todo in list {list_id}");
}

pub async fn get_list(Path(list_id): Path<String>) {
	println!("Getting todos in list {list_id}");
}

pub async fn get_item(Path((list_id, todo_id)): Path<(String, String)>) {
	println!("Getting todo {todo_id} in list {list_id}");
}

pub async fn update(Path((list_id, todo_id)): Path<(String, String)>) {
	println!("Updating todo {todo_id} in list {list_id}");
}

pub async fn delete(Path((list_id, todo_id)): Path<(String, String)>) {
	println!("Deleting todo {todo_id} in list {list_id}");
}
