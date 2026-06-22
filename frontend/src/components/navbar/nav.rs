use leptos::{IntoView, component, prelude::*, view};

use crate::{components::navbar::theme::ThemeToggle, store::auth::get_current_user};

#[component]
pub fn Navbar() -> impl IntoView {
    let current_user = get_current_user();

    view! {
        <nav class="navbar bg-base-100 shadow-sm">
            <div class="flex-1">
                <a class="btn btn-ghost text-xl" href="/">"VaultChat"</a>
            </div>
            <div class="flex-none">
                <Show
                    when=move || current_user.get().is_some()
                    fallback=|| view! {
                        <a href="/login" class="btn btn-primary">"Login"</a>
                    }
                >
                    <div class="flex items-center gap-4">
                        <span>"Welcome, " {move || current_user.get().unwrap().pseudo}</span>
                        <button class="btn btn-error btn-sm">"Logout"</button>
                    </div>
                </Show>
                <ThemeToggle />
            </div>
        </nav>
    }
}
