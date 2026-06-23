use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};

use crate::{pages::login::helpers::login, store::auth::set_current_user};

#[component]
pub fn Login() -> impl IntoView {
    let navigate = use_navigate();
    let (pseudo, set_pseudo) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let login = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let user = login(pseudo.get().as_str(), password.get().as_str());
        match user {
            Some(_) => {
                set_current_user(user);
                navigate("/chat", Default::default());
            }
            None => {}
        }
    };

    let handle_pseudo_change = move |ev| {
        set_pseudo.set(event_target_value(&ev));
    };

    let handle_password_change = move |ev| {
        set_password.set(event_target_value(&ev));
    };

    let is_button_disabled = move || pseudo.get().trim().is_empty() || password.get().is_empty();

    view! {
        <div class="min-h-screen flex items-center justify-center bg-base-200">

            <form
                class="flex flex-col gap-4 w-full max-w-sm p-8 bg-base-100 rounded-xl shadow-lg"
                on:submit=login
            >
                <h2 class="text-2xl font-bold text-center mb-2">"VaultChat"</h2>

                <input
                    type="text"
                    placeholder="Pseudo"
                    class="input input-bordered input-primary w-full"
                    prop:value=pseudo
                    on:input=handle_pseudo_change
                />
                <input
                    type="password"
                    placeholder="Password"
                    class="input input-bordered input-primary w-full"
                    prop:value=password
                    on:input=handle_password_change
                />

                <button type="submit" class="btn btn-success w-full" prop:disabled=is_button_disabled>
                    "Login"
                </button>

                <div class="divider">"OR"</div>

                <A attr:class="btn btn-soft btn-info w-full" href="/register">
                    "Register"
                </A>
            </form>
        </div>
    }
}
