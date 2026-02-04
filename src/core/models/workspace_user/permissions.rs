use std::collections::HashMap;

use crate::{bitmask_serde, core::models::ids::*};
use ::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UserPermissions {
    pub workspace: WorkspacePermissions,
    pub workspace_users: HashMap<WorkspaceUserId, WorkspaceUsersPermissions>,
    pub bases: HashMap<BaseId, BaseSubPermissions>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BaseSubPermissions {
    pub base: BasePermissions,
    pub tables: HashMap<TableId, TableSubPermissions>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TableSubPermissions {
    pub table: TablePermissions,
    pub fields: HashMap<FieldId, FieldPermissions>,
    pub records: HashMap<RecordId, RecordPermissions>,
}

// so for permissions we will need
// can() user do stuff,
// change permissions,
// and presets, like, for new users what they can do, or just for guests what they can do etc
// (we give the workspace role)

bitmask! {
    pub mask WorkspacePermissions: u32 where flags WorkspacePermission {
        Edit = 0x1,
        Delete = 0x2,
        ManageRoles = 0x4,
        ManageUsers = 0x8,
        ExportData = 0x10,
        ManageIntegrations = 0x20,
        ViewAuditLogs = 0x40,
    }
}

bitmask! {
    pub mask WorkspaceUsersPermissions: u32 where flags WorkspaceUsersPermission {
        Invite = 0x1,
        Desactivate = 0x2,
        Activate = 0x4,
        Ban = 0x8,
        Promote = 0x10,
        Demote = 0x20,
        ViewDeletedUsers = 0x40,
    }
}

bitmask! {
    pub mask BasePermissions: u32 where flags BasePermission {
        View = 0x1,
        Edit = 0x2,
        Delete = 0x4,
        ManageTables = 0x8,
        ManageViews = 0x10,
        ManageUsers = 0x20,
    }
}

bitmask! {
    pub mask TablePermissions: u32 where flags TablePermission {
        CreateRecords = 0x1,
        EditRecords = 0x2,
        DeleteRecords = 0x4,
        ViewRecords = 0x8,
        CreateFields = 0x10,
        EditFields = 0x20,
        DeleteFields = 0x40,
        ViewFields = 0x80,
        CreateViews = 0x100,
        EditViews = 0x200,
        DeleteViews = 0x400,
        ViewViews = 0x800,
        Archive = 0x1000,
        Edit = 0x2000,
        Delete = 0x4000,
        View = 0x8000,
        BulkImport = 0x10000,
        LockSchema = 0x20000,
        ExportTable = 0x40000,
    }
}

bitmask! {
    pub mask FieldPermissions: u32 where flags FieldPermission {
        View = 0x1,
        Edit = 0x2,
        Delete = 0x4,
        AddFormula = 0x8,
        HideFromSearch = 0x10,
    }
}

bitmask! {
    pub mask RecordPermissions: u32 where flags RecordPermission {
        View = 0x1,
        Edit = 0x2,
        Delete = 0x4,
        Comment = 0x8,
        Share = 0x10,
    }
}

bitmask_serde!(WorkspacePermissions);
bitmask_serde!(WorkspaceUsersPermissions);
bitmask_serde!(BasePermissions);
bitmask_serde!(TablePermissions);
bitmask_serde!(FieldPermissions);
bitmask_serde!(RecordPermissions);
