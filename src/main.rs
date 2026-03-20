// Source - https://stackoverflow.com/a/25877389
// Posted by Arjan, modified by community. See post 'Timeline' for change history
// Retrieved 2026-01-26, License - CC BY-SA 4.0

#![allow(unexpected_cfgs)]

#[cfg(test)]
mod test;

mod app;
mod core;
use crate::core::db;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chacha20poly1305::Key;
use dotenv::dotenv;
use hackclub_auth_api::HCAuth;
use std::sync::LazyLock;

pub static HCAUTH: LazyLock<HCAuth> = LazyLock::new(|| {
    HCAuth::new(
        dotenv!("CLIENT_ID"),
        dotenv!("CLIENT_SECRET"),
        dotenv!("REDIRECT_URI"),
    )
});

pub static MASTER_KEY: LazyLock<Key> = LazyLock::new(|| {
    dotenv().ok();

    let key_hex = std::env::var("MASTER_KEY").expect("MASTER_KEY environment variable not set");

    let key_bytes = hex::decode(&key_hex).expect("MASTER_KEY must be valid hex string");

    if key_bytes.len() != 32 {
        panic!("MASTER_KEY must be exactly 32 bytes (64 hex characters)");
    }

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);
    Key::from(key_array)
});

#[macro_use]
extern crate bitmask;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    db::init().await;
    let app = Router::new().route("/", get(/*method_router*/));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9898")
        .await
        .unwrap();
    axum::serve(listener, app);
}
