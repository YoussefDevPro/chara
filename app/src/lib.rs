use crate::codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use leptos_use::SameSite;
use leptos_use::UseCookieOptions;
use leptos_use::use_cookie;
use leptos_use::use_cookie_with_options;

#[component]
pub fn Dashboard() -> impl IntoView {
    // 1. We wrap this in a way that handles the SSR vs Client difference.
    // use_cookie is designed to look at Req headers on server and document.cookie on client.
    let (token, set_token) = use_cookie::<String, FromToStringCodec>("token");

    view! {
        <div class="p-8">
            // 2. Use a closure for reactivity.
            // On the server, token.get() will pull from the Axum request headers.
            {move || match token.get() {
                Some(t) => view! {
                    <div class="bg-blue-50 p-6 rounded-lg shadow">
                        <h2 class="text-xl font-bold">"Welcome to your Dashboard"</h2>
                        <p class="mt-2 text-sm text-gray-600">"Your session: " {t}</p>

                        <button
                            class="mt-4 px-4 py-2 bg-red-500 text-white rounded"
                            on:click=move |_| {
                                // This only runs on the client (browser), so it's safe!
                                set_token.set(None);
                                #[cfg(feature = "hydrate")]
                                {
                                   let _ = window().location().set_href("/");
                                }
                            }
                        >
                            "Logout"
                        </button>
                    </div>
                }.into_view(),
                None => view! {
                    <div class="text-center">
                        <p class="text-red-500">"Not authenticated or Cookie missing!"</p>
                        <a href="/" class="text-blue-500 underline">"Return Home"</a>
                    </div>
                }.into_view()
            }}
        </div>
    }
}

#[server]
pub async fn get_user_dashboard() -> Result<String, ServerFnError> {
    use axum::extract::ConnectInfo;
    use axum::http::HeaderMap;
    use axum::http::header;
    use charac::service::user::*;
    use leptos_axum::extract;
    use std::net::SocketAddr;
    let headers: HeaderMap = extract().await?;
    let ConnectInfo(addr): ConnectInfo<SocketAddr> = extract().await?;

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let headers: HeaderMap = extract().await?;

    // Extract the token from the Cookie header
    let cookie_header = headers
        .get(header::COOKIE)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ServerFnError::new("No cookies found. Please log in."))?;

    let token = cookie_header
        .split(';')
        .find(|s| s.trim().starts_with("token="))
        .map(|s| s.trim().trim_start_matches("token="))
        .ok_or_else(|| ServerFnError::new("Authentication token missing."))?;

    let service = UserService::login(AuthMethod::Session(SessionI {
        token: token.to_string(),
        ip: addr.ip().to_string(),
        agent: user_agent.to_string(),
    }))
    .await
    .ok()
    .ok_or(ServerFnError::new("aaaaaaaaaaaaaa".to_string()))?;

    Ok(format!("Welcome back! {:?}", service.user))
}

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
                // Call the server function
                let result = oauth(p.code).await;

                if result.is_ok() {
                    // Manually navigate on the client side after success
                    #[cfg(feature = "hydrate")]
                    {
                        let _ = window().location().set_href("/dashboard");
                    }
                }
            });
        }
    });

    view! {
        <div class="flex h-screen items-center justify-center">
            <h2>"Authenticating..."</h2>
        </div>
    }
}

#[server(Oauth, "/callback")]
pub async fn oauth(code: String) -> Result<(), ServerFnError> {
    use axum::extract::ConnectInfo;
    use axum::http::{HeaderMap, HeaderValue, header};
    use charac::HCAUTH;
    use charac::service::user::{AuthMethod, UserService};
    use leptos_axum::{ResponseOptions, extract};
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
    let (_, set_token) = use_cookie_with_options::<String, FromToStringCodec>(
        "token",
        UseCookieOptions::default()
            .max_age(604_800_000) // 7 days in ms
            .same_site(SameSite::Lax)
            .path("/")
            .http_only(true),
    );

    set_token.set(Some(session_token));

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
    Ok(charac::HCAUTH.get_oauth_uri(&["openid", "profile", "email"]))
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
                    <Route path=StaticSegment("callback") view=Oauth />
                    <Route path=StaticSegment("dashboard") view=Dashboard />
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
