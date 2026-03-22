#[macro_export]
macro_rules! env_required {
    ($key:expr) => {{
        use std::env;

        match env::var($key) {
            Ok(val) if !val.trim().is_empty() => val,
            _ => panic!(concat!("Missing required env variable: ", $key)),
        }
    }};
}

use std::sync::LazyLock;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

pub mod error;
pub use error::Irror;

// use surrealdb::engine::remote::ws::{Client, Ws};
// use surrealdb::opt::auth::Root;

// pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
pub static DB: LazyLock<Surreal<Db>> = LazyLock::new(Surreal::init);

pub async fn init() {
    DB.connect::<Mem>(()).await.unwrap();
    DB.use_ns(env_required!("DB_NAMESPACE"))
        .use_db("main")
        .await
        .unwrap();
}
