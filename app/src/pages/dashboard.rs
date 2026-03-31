use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::ui::theme_toggle::ThemeToggle;

use leptos::prelude::*;

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
