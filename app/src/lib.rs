use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

#[derive(Params, PartialEq, Clone)]
struct HcaCode {
    code: String,
}

#[component]
pub fn Oauth() -> impl IntoView {
    let params = use_params::<HcaCode>();

    Effect::new(move |_| {
        if let Ok(p) = params.get() {
            spawn_local(async move {
                let _ = oauth(p.code).await;
            });
        }
    });

    view! {
        <div class="flex h-screen items-center justify-center">
            <div class="text-center">
                <h2 class="text-xl font-semibold">"Authenticating..."</h2>
                <p class="text-gray-500">"Please wait while we log you in."</p>
            </div>
        </div>
    }
}
#[server(Oauth, "/callback")]
pub async fn oauth(code: String) -> Result<(), ServerFnError> {
    use axum::extract::ConnectInfo;
    use axum::http::{HeaderMap, HeaderValue, header};
    use leptos_axum::{ResponseOptions, extract};
    use libc::HCAUTH;
    use libc::service::user::{AuthMethod, UserService};
    use std::net::SocketAddr;

    let headers: HeaderMap = extract().await?;
    let ConnectInfo(addr): ConnectInfo<SocketAddr> = extract().await?;
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let auth_res = HCAUTH
        .exchange_code(code)
        .await
        .map_err(|e| ServerFnError::new(e))?;
    let access_token = auth_res
        .access_token
        .ok_or(ServerFnError::new("aaaaaaaaaaaa 69 not nice".to_string()))?;
    let service = UserService::login(AuthMethod::Hca(access_token))
        .await
        .ok()
        .ok_or(ServerFnError::new("&&&&&&&&&&&&&&&&&OOO"))?;
    let session_token = service
        .create_session(addr.ip().to_string(), user_agent)
        .await
        .ok()
        .ok_or(ServerFnError::new("aaaaaaaaaaaaaaaaaaaaaqqqqqqqqqqq"))?;
    let res_options = leptos::prelude::expect_context::<ResponseOptions>();
    let cookie_str = format!(
        "token={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
        session_token,
        60 * 60 * 24 * 7
    );
    res_options.insert_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie_str)
            .ok()
            .ok_or(ServerFnError::new("aaaaaaaaaaaaaaaawwwwwwwwwwwwqqqqqqq"))?,
    );

    leptos_axum::redirect("/dashboard");

    Ok(())
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[server]
pub async fn test() -> Result<String, ServerFnError> {
    use axum::extract::ConnectInfo;
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use std::net::SocketAddr;
    let headers: HeaderMap = extract().await?;
    let ConnectInfo(addr): ConnectInfo<SocketAddr> = extract().await?;

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    Ok(format!(
        "it smh worked ??? here is the user sir! IP: {} USER_AGENT: {}",
        addr.ip(),
        user_agent
    ))
}

#[server]
pub async fn get_oauth_link() -> Result<String, ServerFnError> {
    Ok(libc::HCAUTH.get_oauth_uri(&[
        "openid",
        "profile",
        "email",
        "name",
        "slack_id",
        "verification_status",
    ]))
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/chara.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1 class="text-2xl font-bold text-blue-600 my-8">"CHARA !"</h1>
        <button
            class="text-center text-red-200 mt-30"
            on:click=move |_| spawn_local(async move {
                let url = get_oauth_link().await.unwrap_or("https://example.com".to_string());
                let _ = window().location().set_href(&url);
            })
        >
            "Oauth with Hackclub Auth!"
        </button>
    }
}
