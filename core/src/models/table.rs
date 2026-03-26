use crate::models::ids::*;
use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, SurrealValue};

// A Table represents a logical container within a Base.
//
// Each Table belongs to exactly one Base.
// Only authorized users of the parent Base (typically the workspace owner or admins)
// may modify or soft-delete a Table.

#[derive(Debug, Clone, PartialEq, SurrealValue, Serialize, Deserialize)]
pub struct Table {
    pub id: Option<TableId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_deleted: bool,
    pub name: String,
}

pub struct InsertTable {
    pub(crate) name: String,
}

pub struct TablePatch {
    pub(crate) name: Option<String>,
    pub(crate) is_deleted: Option<bool>,
}

impl Table {
    pub fn from_insert(insert: InsertTable) -> Self {
        Table {
            id: None,
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            is_deleted: false,
            name: insert.name,
        }
    }
}
