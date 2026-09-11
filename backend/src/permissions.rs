use crate::storage::AuthError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "UPPERCASE")]
pub enum ListRole {
	Owner,
	Editor,
	Viewer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListPermission {
	Read,
	Edit,
	AddTask,
	DeleteTask,
	Invite,
	ManageMembers,
	DeleteList,
}

impl ListRole {
	pub fn can(self, permission: ListPermission) -> bool {
		match self {
			ListRole::Owner => true,

			ListRole::Editor => matches!(
				permission,
				ListPermission::Read
					| ListPermission::Edit
					| ListPermission::AddTask
					| ListPermission::DeleteTask
			),

			ListRole::Viewer => matches!(permission, ListPermission::Read),
		}
	}
}
