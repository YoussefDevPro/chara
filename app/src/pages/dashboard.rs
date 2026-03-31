use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::ui::theme_toggle::ThemeToggle;
use leptos::prelude::*;

#[derive(serde::Deserialize, serde::Serialize)]
struct UserBase {
    name: String,
    owner_name: String,
    id: String,
}

#[server]
pub async fn get_user_bases() -> Result<Vec<UserBase>, ServerFnError> {
    use axum_extra::extract::cookie::PrivateCookieJar;
    use leptos_axum::extract_with_state;
    let state = use_context::<server::AppState>()
        .ok_or_else(|| ServerFnError::new("AppState not found in context"))?;

    let jar = extract_with_state::<PrivateCookieJar, server::AppState>(&state)
        .await
        .map_err(|e| ServerFnError::new(format!("Cookie extraction failed: {e:?}")))?;

    let secret = jar
        .get("session")
        .map(|c| c.value().to_string())
        .ok_or_else(|| ServerFnError::new("No session cookie found"))?;
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    let theme = ThemeMode::init();

    view! {
        <div
            class="relative min-h-screen bg-[var(--background)] text-[var(--foreground)]"
            class:dark=move || theme.is_dark()
        >

            <div class="absolute top-4 right-4 p-2">
                <ThemeToggle />
            </div>

        </div>
    }
}
