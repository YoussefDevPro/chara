use icons::BrickWall;
use leptos::prelude::*;

#[component]
pub fn SideNav() -> impl IntoView {
    view! {
        <aside class="fixed top-0 left-0 z-40 w-64 h-full">
            <BrickWall class="size-8 text-black-500"/>
        </aside>
    }
}
