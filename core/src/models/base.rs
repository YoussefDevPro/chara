use crate::models::ids::*;
use surrealdb::types::{Datetime, SurrealValue};

// A Base represents a sub-entity within a Workspace.
//
// Each Base belongs to exactly one Workspace, identified by `workspace`.
// Only authorized users of the parent Workspace (typically the owner or admins)
// are allowed to modify or soft-delete a Base.

#[derive(SurrealValue, Debug, Clone, PartialEq)]
pub struct Base {
    pub id: Option<BaseId>,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
    pub is_deleted: bool,
    pub owner: UserId,
    pub name: String,
}

pub struct InsertBase {
    pub(crate) name: String,
    pub(crate) owner: UserId,
}

pub struct BasePatch {
    pub(crate) is_deleted: Option<bool>,
    pub(crate) name: Option<String>,
}

impl Base {
    pub fn from_insert(insert: InsertBase) -> Self {
        Base {
            id: None,
            created_at: None,
            updated_at: None,
            is_deleted: false,
            name: insert.name,
            owner: insert.owner,
        }
    }
}
