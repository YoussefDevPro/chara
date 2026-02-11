use crate::core::models::ids::*;
use ::serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Deserialize, Serialize)]
pub struct Identity {
    pub id: Option<IdentityId>,
    pub external_user_id: String,
    pub created_at: Datetime,
    pub updated_at: Datetime, // even tho we shouldnt be able to change it
    pub is_deleted: bool,
}

#[derive(Deserialize, Serialize)]
pub struct InsertIdentity {
    pub(crate) external_user_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct IdentityPatch {
    pub(crate) is_deleted: Option<bool>,
}

impl Identity {
    pub fn from_insert(insert: InsertIdentity) -> Self {
        Identity {
            id: None,
            external_user_id: insert.external_user_id,
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            is_deleted: false,
        }
    }

    pub fn apply_patch(&mut self, patch: IdentityPatch) {
        if let Some(v) = patch.is_deleted {
            self.is_deleted = v;
        }
    }
}
