#[cfg(test)]
mod test;

pub mod app;
pub mod core;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use bitmask::bitmask;
use chacha20poly1305::*;
use dotenv::dotenv as denv;
use dotenv_codegen::dotenv;
use hackclub_auth_api::*;
use leptos::prelude::*;
use leptos_axum::generate_route_list;

use crate::app::App;

use surrealdb::opt::PatchOp;

use std::sync::LazyLock;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::types::SurrealValue;
use surrealdb::Surreal;

use core::db::error::Irror;

use serde::*;

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};

pub static HCAUTH: LazyLock<HCAuth> = LazyLock::new(|| {
    HCAuth::new(
        dotenv!("CLIENT_ID"),
        dotenv!("CLIENT_SECRET"),
        dotenv!("REDIRECT_URI"),
    )
});

pub static MASTER_KEY: LazyLock<Key> = LazyLock::new(|| {
    denv().ok();

    let key_hex = std::env::var("MASTER_KEY").expect("MASTER_KEY environment variable not set");

    let key_bytes = hex::decode(&key_hex).expect("MASTER_KEY must be valid hex string");

    if key_bytes.len() != 32 {
        panic!("MASTER_KEY must be exactly 32 bytes (64 hex characters)");
    }

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);
    Key::from(key_array)
});
