// TODO: add support for SurrealValue and work on the base service

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
