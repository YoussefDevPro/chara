use crate::core::db::error::Error;
use crate::core::db::DB;
use crate::core::models::{
    ids::{UserId, WorkspaceId, WorkspaceUserId},
    workspace::Workspace,
    workspace_user::WorkspaceUser,
};
use crate::core::service::errors::WorkspaceError;
use ::serde::Deserialize;
use surrealdb_types::SurrealValue;

#[derive(Debug)]
pub struct WorkspaceService {
    pub workspace_user: WorkspaceUser,
    workspace_user_record_id: WorkspaceUserId,
    pub workspace: Workspace,
    workspace_record_id: WorkspaceId,
}

impl WorkspaceService {
    pub async fn new(user: UserId, workspace: WorkspaceId) -> Result<Self, Error> {
        let mut res = DB
            .query(
                "
            BEGIN TRANSACTION;
            
            -- get the workspace 
            LET $ws = (SELECT * FROM workspace WHERE id = $workspace AND is_deleted = false);
            
            -- get the workspace_user link
            LET $wu = (SELECT * FROM workspace_user 
                       WHERE is_deleted = false 
                       AND user = $user 
                       AND workspace_id = $workspace);
            
            -- return both
            RETURN {
                workspace: $ws[0],
                workspace_user: $wu[0]
            };
            
            COMMIT TRANSACTION;
            ",
            )
            .bind(("user", user))
            .bind(("workspace", workspace.clone()))
            .await?;
        #[derive(SurrealValue)]
        struct NewResult {
            workspace: Option<Workspace>,
            workspace_user: Option<WorkspaceUser>,
        }

        let result: Option<NewResult> = res.take(0)?;
        let result = result.ok_or(Error::Workspace(WorkspaceError::NotFound))?;

        let workspace_thing = result
            .workspace
            .ok_or(Error::Workspace(WorkspaceError::NotFound))?;
        let workspace_user = result
            .workspace_user
            .ok_or(Error::Workspace(WorkspaceError::NotFound))?;

        let workspace_user_record_id = workspace_user
            .id
            .clone()
            .ok_or(Error::Workspace(WorkspaceError::NotFound))?;
        Ok(WorkspaceService {
            workspace_user,
            workspace_user_record_id: workspace_user_record_id.clone(),
            workspace: workspace_thing,
            workspace_record_id: workspace,
        })
    }
    //pub async fn create_base(&self, name: String) -> Result<Self, Error> {}
    //pub async fn delete_base(&self, base: BaseId) -> Result<Self, Error> {}
    //pub async fn open_base(&self, base: BaseId) -> Result<Self, Error> {}
    //pub async fn edit_workspace_user(
    //    &self,
    //    workspace_user: WorkspaceUserId,
    //) -> Result<Self, Error> {
    //}
    //pub async fn delete_workspace_user(
    //    &self,
    //    workspace_user: WorkspaceUserId,
    //) -> Result<Self, Error> {
    //}
    //pub async fn edit_workspace_name(&self, patch: WorkspacePatch) -> Result<Self, Error> {}
}
