use icons::Annoyed;
use leptos::prelude::*;

#[component]
pub fn SideNav() -> impl IntoView {
    view! {
        <aside class="sticky top-0 z-40 w-auto h-screen p-2 border-r bg-background">
            <Annoyed class="size-12 text-black text-[var(--foreground)] stroke-[var(foreground)] border p-2" />
        </aside>
    }
}
