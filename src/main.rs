// Source - https://stackoverflow.com/a/25877389
// Posted by Arjan, modified by community. See post 'Timeline' for change history
// Retrieved 2026-01-26, License - CC BY-SA 4.0

#![allow(unexpected_cfgs)]

use axum::Router;
use chara::app::*;
use leptos_axum::*;

use chara::*;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    core::db::init().await;
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(file_and_error_handler(shell))
        .with_state(leptos_options);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9898")
        .await
        .unwrap();
    println!("running at {addr}");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
