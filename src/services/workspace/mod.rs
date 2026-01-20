use crate::models::ids::*;
use crate::models::session::*;
use crate::models::user::UserRole;
use crate::models::user::*;
use crate::models::workspace;
use crate::models::workspace::*;
use crate::services::user::get_user_by_session_token;
use crate::services::Error;
use crate::services::DB;

const WORKSPACE: &str = "workspace";

// note: use a struct for the name to ensure name saftly, like checking the syntax etc
// oh and also for session lol
pub async fn create_workspace(
    name: String,
    session_token: String,
) -> Result<Option<Workspace>, Error> {
    let user = get_user_by_session_token(&session_token)
        .await?
        .ok_or(Error::NotFound)?;
    match user.role {
        UserRole::Admin => {
            let owner = user.id.ok_or(Error::Forbidden)?;

            let workspace = InsertWorkspace { name, owner };
            DB.create(WORKSPACE)
                .content(Workspace::from_insert(workspace))
                .await?
                .ok_or(Error::AlreadyExists)
        }
        UserRole::User => Err(Error::Forbidden),
    }
}

pub async fn get_workspace_by_id(
    id: &WorkspaceId,
    session_token: &String,
) -> Result<Option<Workspace>, Error> {
    let user: User = get_user_by_session_token(session_token)
        .await?
        .ok_or(Error::NotFound)?;
    let owner = user.id.ok_or(Error::Forbidden)?;
    let workspace: Workspace = DB
        .select((WORKSPACE, id.0.id.to_string()))
        .await?
        .ok_or(Error::NotFound)?;
    if workspace.owner != owner && user.role != UserRole::Admin {
        return Err(Error::NotFound);
    };
    Ok(Some(workspace))
}

pub async fn workspace_exists_for_owner(
    workspace_id: &WorkspaceId,
    owner_id: &UserId,
) -> Result<bool, Error> {
    let mut res = DB
        .query(
            r#"
            SELECT id
            FROM workspace
            WHERE id = $workspace
              AND owner = $owner
            LIMIT 1
            "#,
        )
        .bind(("workspace", workspace_id.0.id.to_string()))
        .bind(("owner", owner_id.0.id.to_string()))
        .await?;

    let row: Option<serde_json::Value> = res.take(0)?;
    Ok(row.is_some())
}

pub async fn update_workspace(
    id: &WorkspaceId,
    session_token: &String,
    patch: WorkspacePatch,
) -> Result<Option<Workspace>, Error> {
    let user = get_user_by_session_token(session_token)
        .await?
        .ok_or(Error::Forbidden)?;
    match user.role {
        UserRole::User => Err(Error::Forbidden),
        UserRole::Admin => {
            let owner = user.id.ok_or(Error::Forbidden)?;

            let mut res = DB
                .query(
                    r#"
        UPDATE workspace
        SET $patch
        WHERE id = $workspace_id
          AND owner = $owner_id
        RETURN AFTER;
        "#,
                )
                .bind(("workspace_id", id.0.id.to_string()))
                .bind(("owner_id", owner.0.id.to_string()))
                .bind(("patch", patch))
                .await?;

            let updated: Option<Workspace> = res.take(0)?;

            match updated {
                Some(ws) => Ok(Some(ws)),
                None => Err(Error::Forbidden),
            }
        }
    }
}
