use std::sync::Arc;

use axum::routing::{Router, delete, get, post, put};

use crate::{authorization::ProtectedRoutes, permissions::ListPermission, storage::Storage};

pub mod list;
pub mod tags;
pub mod tasks;

pub fn lists() -> Router<Arc<dyn Storage>> {
	Router::new()
		.authenticated_route("/", post(list::new))
		.authenticated_route("/", get(list::get_overview))
		.authenticated_route("/join/{invitation_id}", post(list::join))
		.protected_route("/{list_id}", get(list::get), ListPermission::Read)
		.protected_route("/{list_id}", put(list::rename), ListPermission::Edit)
		.protected_route(
			"/{list_id}",
			delete(list::delete),
			ListPermission::DeleteList,
		)
		.protected_route(
			"/{list_id}/share",
			post(list::share),
			ListPermission::Invite,
		)
		.nest("/{list_id}/tags", tags::tags())
		.nest("/{list_id}/tasks", tasks::tasks())
}
