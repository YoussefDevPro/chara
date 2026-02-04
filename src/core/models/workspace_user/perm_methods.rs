use crate::core::models::workspace_user::permissions::*;
use crate::core::models::workspace_user::*;
use std::collections::HashMap;

impl UserPermissions {
    /// Global check for Workspace actions
    pub fn can_workspace(&self, perm: WorkspacePermission) -> bool {
        self.workspace.contains(perm)
    }

    /// Check if user can perform an action on a specific Base
    pub fn can_base(&self, base_id: &BaseId, perm: BasePermission) -> bool {
        self.bases
            .get(base_id)
            .map(|b| b.base.contains(perm))
            .unwrap_or(false)
    }

    /// Check Table access: Validates Base -> Table
    pub fn can_table(&self, base_id: &BaseId, table_id: &TableId, perm: TablePermission) -> bool {
        if self.workspace.contains(WorkspacePermission::Edit) {
            return true;
        }

        self.bases
            .get(base_id)
            .and_then(|b| b.tables.get(table_id))
            .map(|t| t.table.contains(perm))
            .unwrap_or(false)
    }

    /// Check Field access: Validates Base -> Table -> Field override
    pub fn can_field(
        &self,
        base_id: &BaseId,
        table_id: &TableId,
        field_id: &FieldId,
        perm: FieldPermission,
    ) -> bool {
        if self.workspace.contains(WorkspacePermission::Edit) {
            return true;
        }

        let table_sub = self.bases.get(base_id).and_then(|b| b.tables.get(table_id));

        match table_sub {
            Some(ts) => {
                match ts.fields.get(field_id) {
                    Some(f_perm) => f_perm.contains(perm),
                    None => {
                        // Fallback: If no specific field perm, does the table allow viewing/editing fields generally?
                        match perm {
                            FieldPermission::View => ts.table.contains(TablePermission::ViewFields),
                            FieldPermission::Edit => ts.table.contains(TablePermission::EditFields),
                            _ => false,
                        }
                    }
                }
            }
            None => false,
        }
    }

    /// THE BOSS: Has all bits set for Workspace
    pub fn preset_owner() -> Self {
        Self {
            workspace: WorkspacePermissions::all(),
            workspace_users: HashMap::new(),
            bases: HashMap::new(),
        }
    }

    pub fn preset_guest() -> Self {
        Self {
            workspace: WorkspacePermissions::none(),
            workspace_users: HashMap::new(),
            bases: HashMap::new(),
        }
    }
}

impl BaseSubPermissions {
    pub fn empty() -> Self {
        Self {
            base: BasePermissions::none(),
            tables: HashMap::new(),
        }
    }
}

impl TableSubPermissions {
    pub fn empty() -> Self {
        Self {
            table: TablePermissions::none(),
            fields: HashMap::new(),
            records: HashMap::new(),
        }
    }
}
