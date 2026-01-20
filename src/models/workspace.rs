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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacePatch {
    pub name: Option<String>,
}
