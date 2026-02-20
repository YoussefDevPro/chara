// Source - https://stackoverflow.com/a/25877389
// Posted by Arjan, modified by community. See post 'Timeline' for change history
// Retrieved 2026-01-26, License - CC BY-SA 4.0

#![allow(unexpected_cfgs)]

mod core;
use hackclub_auth_api::HCAuth;
use std::sync::LazyLock;

pub static HCAUTH: LazyLock<HCAuth> = LazyLock::new(|| {
    HCAuth::new(
        dotenv!("CLIENT_ID"),
        dotenv!("CLIENT_SECRET"),
        dotenv!("REDIRECT_URI"),
    )
});

#[macro_use]
extern crate bitmask;

#[macro_use]
extern crate dotenv_codegen;

fn main() {
    println!("Hello, world!");
}
