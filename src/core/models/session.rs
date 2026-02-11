use crate::core::models::ids::*;
use ::serde::{Deserialize, Serialize};
use chrono::Duration;
use surrealdb::sql::Datetime;

#[derive(Deserialize, Serialize)]
pub struct Session {
    pub id: Option<SessionId>,
    pub token: String,
    pub ip: String,
    pub user_agent: String,
    pub created_at: Datetime,
    pub expires_at: Datetime, // make surrealdb drop the session if it expires
}

#[derive(Deserialize, Serialize)]
pub struct InsertSession {
    pub token: String,
    pub ip: String,
    pub user_agent: String,
}

// no session patch bc we're not supposed to change this at all

impl Session {
    pub fn from_insert(insert: InsertSession) -> Self {
        Session {
            id: None,
            token: insert.token,
            ip: insert.ip,
            user_agent: insert.user_agent,
            created_at: Datetime::from(chrono::Utc::now()),
            expires_at: Datetime::from(chrono::Utc::now() + Duration::days(3)), // hardened expiration :p
        }
    }
}
