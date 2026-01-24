use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::models::ids::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Option<WorkspaceId>,
    pub owner: UserId,
    pub name: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_soft_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertWorkspace {
    pub owner: UserId,
    pub name: String,
}

impl Workspace {
    pub fn from_insert(input: InsertWorkspace) -> Self {
        Self {
            id: None,
            owner: input.owner,
            name: input.name,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
            is_soft_deleted: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacePatch {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminWorkspacePatch {
    pub is_soft_deleted: bool,
}

bitmask! {
    pub mask Permissions: u64 where flags Permission {
        ManageUsers  = 0b0000000000000001, // 1
        InviteUsers  = 0b0000000000000010, // 2
        RemoveUsers  = 0b0000000000000100, // 4
        ChangeRoles  = 0b0000000000001000, // 8
        CreateBases  = 0b0000000000010000, // 16
        DeleteBases  = 0b0000000000100000, // 32
    }
} // thats how permissions should looks like, well, ig thats all for today in this project, new
  // todo list, new week full of pain lol
