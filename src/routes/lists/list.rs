use axum::extract::Path;

pub async fn new() {
	println!("Creating new list");
}

pub async fn update(Path(list_id): Path<String>) {
	println!("Updating list {list_id}");
}

pub async fn get(Path(list_id): Path<String>) {
	println!("Getting list {list_id}");
}

pub async fn delete(Path(list_id): Path<String>) {
	println!("Deleting list {list_id}");
}
