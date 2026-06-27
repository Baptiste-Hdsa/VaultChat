use leptos::{IntoView, component, prelude::*, view};
use leptos_router::components::A;

use crate::{
    components::navbar::{logout::LogoutBtn, theme::ThemeToggle},
    store::auth::get_current_user,
};

#[component]
pub fn Navbar() -> impl IntoView {
    let current_user = get_current_user();

    view! {
        <nav class="navbar bg-base-100 shadow-sm shrink-0">
            <div class="flex-1">
                <A attr:class="btn btn-ghost text-xl" href="/">"VaultChat"</A>
            </div>
            <div class="flex">
                <Show
                    when=move || current_user.get().is_some()
                    fallback=|| view! {
                        <A href="/login" attr:class="btn btn-primary">"Login"</A>
                    }
                >
                    <div class="flex items-center gap-4">
                        <span>{move || current_user.get().unwrap().username}</span>
                        <A href="/chat" attr:class="btn btn-success">"Go to chats"</A>
                        <LogoutBtn/>
                    </div>
                </Show>
                <ThemeToggle />
            </div>
        </nav>
    }
}
