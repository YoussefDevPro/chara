use std::collections::HashMap;

use crate::models::ids::*;
use surrealdb::types::{Datetime, SurrealValue};

mod cell;
use crate::models::record::cell::*;

#[derive(Debug, Clone, PartialEq, Eq, SurrealValue)]
pub struct Record {
    id: Option<RecordId>,
    created_at: Option<Datetime>,
    updated_at: Option<Datetime>,
    is_deleted: bool,
    cells: HashMap<String, CellValue>, // K: FieldId
    table: TableId,
}

pub struct InsertRecord {
    pub(crate) table: TableId,
    pub(crate) cells: HashMap<String, CellValue>,
}

pub struct RecordPatch {
    pub(crate) changed_cells: Option<Vec<(String, CellValue)>>,
}

impl Record {
    pub fn from_insert(insert: InsertRecord) -> Self {
        Record {
            id: None,
            created_at: None,
            updated_at: None,
            is_deleted: false,
            cells: insert.cells,
            table: insert.table,
        }
    }
}
