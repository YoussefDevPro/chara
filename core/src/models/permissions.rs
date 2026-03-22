use crate::{bitmask_serde, relation};
use bitmask::bitmask;
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

bitmask! {
    pub mask BasePermissions: i32 where flags BasePermission {
        Admin = 1 << 0,
        View = 1 << 1,
        Edit = 1 << 2,
        Delete = 1 << 3,

        ManageTables = 1 << 4,
        ManageViews = 1 << 5,
        ManagerUserPermissions = 1 << 6,
        ManageAutomatisations = 1 << 7,
        ManageInvitations = 1 << 8,
    }
}
bitmask_serde!(BasePermissions);
relation!(CanAccessBase, BasePermissions);

bitmask! {
    pub mask TablePermissions: i32 where flags TablePermission {
        Admin = 1 << 0,
        View = 1 << 1,
        Edit = 1 << 2,
        Delete = 1 << 3,

        BulkImport = 1 << 4,
        LockFields = 1 << 5,
        Export = 1 << 6,
        Archive = 1 << 7
    }
}
bitmask_serde!(TablePermissions);
relation!(CanAccessTable, TablePermissions);

bitmask! {
    pub mask FieldPermissions: i32 where flags FieldPermission {
        Admin = 1 << 0,
        View = 1 << 1,
        Edit = 1 << 2,
        Delete = 1 << 3,

        Comment = 1 << 4,
        Lock = 1 << 5,
        // XXXXXX = 1 << 6,
        // XXXXXX = 1 << 7
    }
}
bitmask_serde!(FieldPermissions);
relation!(CanAccessField, FieldPermissions);

#[macro_export]
macro_rules! relation {
    ( $( $x:ident ,$y:ident), * ) => {
        #[derive(Deserialize, Serialize, PartialEq, Eq, SurrealValue)]
        $(pub struct $x {
            pub perm: $y,
        })*
    };
}

#[macro_export]
macro_rules! bitmask_serde {
    ($ty:ident) => {
        impl serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_i32(self.mask)
            }
        }

        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let mask = i32::deserialize(deserializer)?;
                Ok($ty { mask })
            }
        }

        impl From<i32> for $ty {
            fn from(mask: i32) -> Self {
                $ty { mask }
            }
        }

        impl From<$ty> for i32 {
            fn from(val: $ty) -> i32 {
                val.mask
            }
        }

        impl SurrealValue for $ty {
            fn kind_of() -> surrealdb::types::Kind {
                surrealdb::types::Kind::Number
            }

            fn into_value(self) -> surrealdb::types::Value {
                surrealdb::types::Value::Number(surrealdb::types::Number::Int(self.mask as i64))
            }

            fn from_value(value: surrealdb::types::Value) -> Result<Self, surrealdb::Error> {
                match value {
                    surrealdb::types::Value::Number(num) => match num {
                        surrealdb::types::Number::Int(i) => Ok($ty { mask: i as i32 }),
                        surrealdb::types::Number::Float(f) => Ok($ty { mask: f as i32 }),
                        _ => Err(surrealdb::Error::thrown(
                            "Unsupported number type for bitmask".to_string(),
                        )),
                    },
                    _ => Err(surrealdb::Error::thrown(
                        "Expected a numeric value for bitmask".to_string(),
                    )),
                }
            }

            fn is_value(value: &surrealdb::types::Value) -> bool {
                matches!(value, surrealdb::types::Value::Number(_))
            }
        }
    };
}
