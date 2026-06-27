use leptos::{prelude::*, task::spawn_local};
use leptos_icons::Icon;
use leptos_router::{components::A, hooks::use_navigate};

use crate::{data_models::user::User, pages::login::helpers::login_request};

#[component]
pub fn Login() -> impl IntoView {
    let navigate = use_navigate();
    let (pseudo, set_pseudo) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let set_global_user = use_context::<WriteSignal<Option<User>>>()
        .expect("Auth context is missing from the app root");

    let (auth_error, set_auth_error) = signal(Option::<String>::None);

    let login = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        set_auth_error.set(None);

        let pseudo_val = pseudo.get();
        let password_val = password.get();
        let navigate_clone = navigate.clone();
        spawn_local(async move {
            let user_result = login_request(&pseudo_val, &password_val).await;
            match user_result {
                Some(user) => {
                    set_global_user.set(Some(user));
                    navigate_clone("/chat", Default::default());
                }
                None => {
                    set_auth_error.set(Some("Invalid pseudo or password".to_string()));
                }
            }
        });
    };

    let handle_pseudo_change = move |ev| {
        set_pseudo.set(event_target_value(&ev));
        set_auth_error.set(None);
    };

    let handle_password_change = move |ev| {
        set_password.set(event_target_value(&ev));
        set_auth_error.set(None);
    };

    let is_button_disabled = move || pseudo.get().trim().is_empty() || password.get().is_empty();

    view! {
        <div class="min-h-screen flex items-center justify-center bg-base-200">

            <form
                class="flex flex-col gap-4 w-full max-w-sm p-8 bg-base-100 rounded-xl shadow-lg"
                on:submit=login
            >
                <h2 class="text-2xl font-bold text-center mb-2">"VaultChat"</h2>

                <Show when=move || auth_error.get().is_some() fallback=|| {}>
                    <div class="alert alert-error p-3 text-sm shadow-sm rounded-lg flex items-center">
                        <Icon icon=icondata::BiErrorAltSolid attr:class="size-5 shrink-0" />
                        <span>{move || auth_error.get().unwrap_or_default()}</span>
                    </div>
                </Show>

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
