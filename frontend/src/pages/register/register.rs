use gloo_timers::future;
use leptos::{prelude::*, task::spawn_local};
use leptos_icons::Icon;
use leptos_router::{components::A, hooks::use_navigate};
use leptos_use::signal_debounced;

use crate::{
    pages::register::helpers::*,
    services::{
        keys::{
            bytes_to_base64, derive_kek, export_public_key, generate_rsa_keypair, wrap_private_key,
        },
        web::base_url,
    },
};

#[derive(serde::Serialize)]
struct RegisterPayload {
    username: String,
    password: String, // Kept to hash on backend for standard auth
    public_key: String,
    wrapped_private_key: String,
    crypto_salt: String,
    aes_iv: String,
}

#[component]
pub fn Register() -> impl IntoView {
    let navigate = use_navigate();
    let (should_redirect, set_should_redirect) = signal(false);

    // Variables
    let (pseudo, set_pseudo) = signal(String::new());
    let debounced_pseudo: Signal<String> = signal_debounced(pseudo, 500.0);
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());

    // Error variables
    let (pseudo_errors, set_pseudo_errors) = signal(Vec::<String>::new());
    let (password_errors, set_password_errors) = signal(Vec::<String>::new());
    let (confirm_password_errors, set_confirm_password_errors) = signal(false);
    let (server_error, set_server_error) = signal(Option::<String>::None);
    let (register_success, set_register_success) = signal(false);

    // Dynamic variables
    let is_button_disabled = move || {
        let has_ui_errors = !pseudo_errors.get().is_empty()
            || !password_errors.get().is_empty()
            || confirm_password_errors.get();

        let is_empty = pseudo.get().trim().is_empty()
            || password.get().is_empty()
            || confirm_password.get().is_empty();

        has_ui_errors || is_empty
    };

    // Handle redirect when registration is successful
    Effect::new(move |_| {
        if should_redirect.get() {
            let navigate_clone = navigate.clone();
            spawn_local(async move {
                future::TimeoutFuture::new(2000).await;
                navigate_clone("/login", Default::default());
            });
        }
    });

    // Helpers "functions"
    let register = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_server_error.set(None);

        let pseudo_val = pseudo.get();
        let password_val = password.get();

        spawn_local(async move {
            // A. Generate random parameters
            let mut salt = [0u8; 16];
            let mut iv = [0u8; 12];
            let crypto = web_sys::window().unwrap().crypto().unwrap();
            crypto.get_random_values_with_u8_array(&mut salt).unwrap();
            crypto.get_random_values_with_u8_array(&mut iv).unwrap();

            // B. Run the heavy cryptography!
            // (Make sure to import these functions from your keys.rs module)
            let (pub_key, priv_key) = generate_rsa_keypair().await.unwrap();
            let kek = derive_kek(&password_val, &salt).await.unwrap();

            let wrapped_priv_key_bytes = wrap_private_key(&priv_key, &kek, &iv).await.unwrap();
            let pub_key_b64 = export_public_key(&pub_key).await.unwrap();

            // C. Convert raw bytes to Base64 to send as strings in JSON
            let salt_b64 = bytes_to_base64(&salt);
            let iv_b64 = bytes_to_base64(&iv);
            let wrapped_priv_b64 = bytes_to_base64(&wrapped_priv_key_bytes);

            // D. Send the payload
            let payload = RegisterPayload {
                username: pseudo_val,
                password: password_val,
                public_key: pub_key_b64,
                wrapped_private_key: wrapped_priv_b64,
                crypto_salt: salt_b64,
                aes_iv: iv_b64,
            };

            let client = reqwest::Client::new();
            match client
                .post(base_url() + "/api/users")
                .json(&payload)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        // Registration successful
                        set_register_success.set(true);

                        // Redirect to login after a short delay to show success message
                        set_should_redirect.set(true);
                    } else {
                        // Registration failed
                        let error_msg = match response.text().await {
                            Ok(text) => format!("Registration failed: {}", text),
                            Err(_) => "Registration failed".to_string(),
                        };
                        set_server_error.set(Some(error_msg));
                    }
                }
                Err(err) => {
                    // Network error
                    set_server_error.set(Some(format!("Network error: {:?}", err)));
                }
            };

            drop(priv_key);
        });
    };

    // Change handlers
    let handle_pseudo_change = move |ev| {
        set_pseudo.set(event_target_value(&ev));
        update_errors(
            check_credentials(
                debounced_pseudo.get().as_str(),
                password.get().as_str(),
                confirm_password.get().as_str(),
            ),
            set_pseudo_errors,
            set_password_errors,
            set_confirm_password_errors,
        );
    };

    let handle_password_change = move |ev| {
        set_password.set(event_target_value(&ev));
        update_errors(
            check_credentials(
                pseudo.get().as_str(),
                password.get().as_str(),
                confirm_password.get().as_str(),
            ),
            set_pseudo_errors,
            set_password_errors,
            set_confirm_password_errors,
        );
    };

    let handle_confirm_password_change = move |ev| {
        set_confirm_password.set(event_target_value(&ev));
        update_errors(
            check_credentials(
                pseudo.get().as_str(),
                password.get().as_str(),
                confirm_password.get().as_str(),
            ),
            set_pseudo_errors,
            set_password_errors,
            set_confirm_password_errors,
        );
    };

    // View
    view! {
        <div class="min-h-full flex items-center justify-center bg-base-200 p-4">

            <form
                class="flex flex-col gap-4 w-full max-w-sm p-8 bg-base-100 rounded-xl shadow-lg"
                on:submit=register
            >
                <h2 class="text-2xl font-bold text-center mb-2">"VaultChat"</h2>

                <Show when=move || register_success.get() fallback=|| {}>
                    <div class="alert alert-success p-3 text-sm shadow-sm rounded-lg flex items-center">
                        <Icon icon=icondata::BiCheckCircleRegular attr:class="size-5 shrink-0" />
                        <span>"Account created! Redirecting to login..."</span>
                    </div>
                </Show>

                <Show when=move || server_error.get().is_some() fallback=|| {}>
                    <div class="alert alert-error p-3 text-sm shadow-sm rounded-lg flex items-center">
                        <Icon icon=icondata::BiErrorAltSolid attr:class="size-5 shrink-0" />
                        <span>{move || server_error.get().unwrap_or_default()}</span>
                    </div>
                </Show>

                <div class="flex flex-col gap-1 w-full">
                    <input
                        type="text"
                        placeholder="Pseudo"
                        class="input input-bordered input-primary w-full"
                        prop:value=pseudo
                        on:input=handle_pseudo_change
                    />
                    <Show when=move || !pseudo_errors.get().is_empty() fallback=|| {}>
                        <div class="alert alert-error p-3 mt-1 rounded-lg shadow-sm">
                            <ul class="list-disc list-inside text-xs font-medium space-y-1">
                                {move || {
                                    pseudo_errors
                                        .get()
                                        .into_iter()
                                        .map(|err| view! { <li>{err}</li> })
                                        .collect_view()
                                }}
                            </ul>
                        </div>
                    </Show>
                </div>

                <div class="flex flex-col gap-1 w-full">
                    <input
                        type="password"
                        placeholder="Password"
                        class="input input-bordered input-primary w-full"
                        prop:value=password
                        on:input=handle_password_change
                    />
                    <Show when=move || !password_errors.get().is_empty() fallback=|| {}>
                        <div class="alert alert-error p-3 mt-1 rounded-lg shadow-sm">
                            <ul class="list-disc list-inside text-xs font-medium space-y-1">
                                {move || {
                                    password_errors
                                        .get()
                                        .into_iter()
                                        .map(|err| view! { <li>{err}</li> })
                                        .collect_view()
                                }}
                            </ul>
                        </div>
                    </Show>
                </div>

                <div class="flex flex-col gap-1 w-full">
                    <input
                        type="password"
                        placeholder="Confirm password"
                        class="input input-bordered input-primary w-full"
                        prop:value=confirm_password
                        on:input=handle_confirm_password_change
                    />
                    <Show when=move || confirm_password_errors.get() fallback=|| {}>
                        <div class="alert alert-error p-3 mt-1 rounded-lg shadow-sm">
                            <ul class="list-disc list-inside text-xs font-medium space-y-1">
                                {move || {
                                    view! { <li>{"Passwords do not match"}</li> }
                                }}
                            </ul>
                        </div>
                    </Show>
                </div>

                <button type="submit" class="btn btn-success w-full" prop:disabled=is_button_disabled>
                    "Register"
                </button>

                <div class="divider">"OR"</div>

                <A attr:class="btn btn-soft btn-info w-full" href="/login">
                    "Login"
                </A>
            </form>
        </div>
    }
}
