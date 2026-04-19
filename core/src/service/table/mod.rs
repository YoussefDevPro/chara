use crate::db::*;
use crate::models::*;
use crate::service::errors::TableError;
use crate::service::table::migration::MigrationStrategy;
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

// TODO: gotta work here :sob:
// fr :noooooovanish:

#[derive(Debug, Clone)]
pub struct TableService {
    pub table: Table,
    pub user: UserId,
    pub base: BaseId,
    table_record_id: TableId,
}

// NOTE: FR stands for frontend :p
#[derive(Serialize, Deserialize, SurrealValue)]
pub struct FieldConfigFR {
    pub is_deleted: bool,
    pub config: FieldConfig,
    pub is_primary: bool,
    pub is_nullable: bool,
    pub is_unique: bool,
    pub name: String,
    pub description: Option<String>,
}

impl TableService {
    pub async fn new(tablee: TableId, base: BaseId, user: UserId) -> Result<Self, Irror> {
        let mut res = DB.query("
            LET $is_owner = (SELECT VALUE owner FROM $base)[0] == $user;
            
            SELECT * FROM $table_id WHERE is_deleted = false AND (
                $is_owner OR 
                fn::can(
                    (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $this.id)[0], 
                    2
                )
            );
        ")
        .bind(("user", user.clone()))
        .bind(("base", base.clone()))
        .bind(("table_id", tablee.clone()))
        .await?;

        let table: Table = res.take::<Option<Table>>(1)?.ok_or(TableError::NotFound)?;

        Ok(Self {
            table,
            user,
            base,
            table_record_id: tablee,
        })
    }

    pub async fn get_field_config(&self, field_id: FieldId) -> Result<FieldConfigFR, Irror> {
        let mut res = DB
        .query("
            SELECT * FROM type::thing('field', $field) 
            WHERE 
                table = $table_id AND 
                table.base = $base_id AND 
                is_deleted = false AND
                (
                    (SELECT VALUE owner FROM $base_id)[0] == $user OR
                    fn::can(
                        (SELECT VALUE perms FROM can_access_field WHERE in = $user AND out = $this.id)[0], 
                        2
                    )
                )
        ")
        .bind(("field", field_id))
        .bind(("table_id", self.table_record_id.clone()))
        .bind(("base_id", self.base.clone()))
        .bind(("user", self.user.clone()))
        .await?;
        dbg!(&res);

        let field_config: Option<FieldConfigFR> = res.take(0)?;

        match field_config {
            Some(config) => Ok(config),
            None => Err(Irror::Table(TableError::NotFound)),
        }
    }
    pub async fn create_field(&self, field: InsertField) -> Result<Field, Irror> {
        let field = Field::from_insert(field);
        let mut res = DB
            .query(
                "
            LET $is_owner = (SELECT VALUE owner FROM $base_id)[0] == $user;
            LET $has_table_edit = fn::can(
                (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $table_id)[0], 
                4
            );

            IF $is_owner OR $has_table_edit THEN
                (CREATE field SET 
                    name = $data.name,
                    table = $table_id,
                    is_primary = $data.is_primary,
                    is_nullable = $data.is_nullable,
                    is_unique = $data.is_unique,
                    order = $data.order,
                    description = $data.description,
                    config = $data.config,
                    created_at = time::now(),
                    updated_at = time::now()
                )
            END;
        ",
            )
            .bind(("user", self.user.clone()))
            .bind(("base_id", self.base.clone()))
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("data", field))
            .await?;
        dbg!(&res);

        let created_field: Option<Field> = res.take(2)?;

        match created_field {
            Some(f) => Ok(f),
            None => Err(Irror::Table(TableError::CreateFailed)),
        }
    }
    pub async fn update_field(
        &self,
        field_id: FieldId,
        field: InsertField,
    ) -> Result<Result<Field, MigrationStrategy>, Irror> {
        let field = Field::from_insert(field);
        let mut res = DB
            .query("SELECT * FROM type::thing('field', $field) WHERE table = $table_id")
            .bind(("field", field_id.clone()))
            .bind(("table_id", self.table_record_id.clone()))
            .await?;

        let current_field: Field = res
            .take::<Option<Field>>(0)?
            .ok_or(Irror::Table(TableError::NotFound))?;

        let mut perm_res = DB
            .query(
                "
        SELECT VALUE 
            (SELECT VALUE owner FROM $base_id)[0] == $user OR
            fn::can(
                (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $table_id)[0], 
                4
            )
            ",
            )
            .bind(("base_id", self.base.clone()))
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("user", self.user.clone()))
            .await?;

        let is_authorized: bool = perm_res.take::<Option<bool>>(0)?.unwrap_or(false);
        if !is_authorized {
            return Err(Irror::Table(TableError::Unauthorized));
        }

        let strategy = current_field.config.get_migration_strategy(&field.config);

        if strategy == MigrationStrategy::Risky || strategy == MigrationStrategy::Destructive {
            return Ok(Err(strategy));
        }

        let mut update_res = DB
            .query("UPDATE type::thing('field', $field) CONTENT $field_config")
            .bind(("field", field_id))
            .bind(("field_config", field))
            .await?;

        let updated: Field = update_res
            .take::<Option<Field>>(0)?
            .ok_or(Irror::Table(TableError::NotFound))?;

        Ok(Ok(updated))
    }
}
