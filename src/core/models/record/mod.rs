use std::collections::HashMap;

use crate::core::models::ids::*;
use ::serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

mod cell;
use crate::core::models::record::cell::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Record {
    id: Option<RecordId>,
    created_at: Datetime,
    updated_at: Datetime,
    is_deleted: bool,
    cells: HashMap<FieldId, CellValue>,
    table: TableId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InsertRecord {
    pub(crate) table: TableId,
    pub(crate) cells: HashMap<FieldId, CellValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecordPatch {
    pub(crate) changed_cells: Option<Vec<(FieldId, CellValue)>>,
}

impl Record {
    pub fn from_insert(insert: InsertRecord) -> Self {
        Record {
            id: None,
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            is_deleted: false,
            cells: insert.cells,
            table: insert.table,
        }
    }

    pub fn apply_patch(&mut self, patch: RecordPatch) {
        if let Some(v) = patch.changed_cells {
            for (field_id, cell_value) in v {
                self.cells
                    .entry(field_id)
                    .and_modify(|existing| *existing = cell_value.clone())
                    .or_insert(cell_value);
            }
        };
    }
}
