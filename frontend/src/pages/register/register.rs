use leptos::prelude::*;
use leptos_use::signal_debounced;

use crate::pages::register::helpers::*;

#[component]
pub fn Register() -> impl IntoView {
    // Variables
    let (pseudo, set_pseudo) = signal(String::new());
    let debounced_pseudo: Signal<String> = signal_debounced(pseudo, 500.0);
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());

    // Error variables
    let (pseudo_errors, set_pseudo_errors) = signal(Vec::<String>::new());
    let (password_errors, set_password_errors) = signal(Vec::<String>::new());
    let (confirm_password_errors, set_confirm_password_errors) = signal(false);

    // Dynamic variables
    let is_pseudo_taken = move || is_pseudo_taken(debounced_pseudo.get());

    let is_button_disabled = move || {
        let has_ui_errors = !pseudo_errors.get().is_empty()
            || !password_errors.get().is_empty()
            || confirm_password_errors.get();

        let is_empty = pseudo.get().trim().is_empty()
            || password.get().is_empty()
            || confirm_password.get().is_empty();

        has_ui_errors || is_empty
    };

    // Helpers "functions"
    let register = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        register(pseudo.get().as_str(), password.get().as_str());
    };

    // Change handlers
    let handle_pseudo_change = move |ev| {
        set_pseudo.set(event_target_value(&ev));
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
        <div class="min-h-screen flex items-center justify-center bg-base-200">

            <form
                class="flex flex-col gap-4 w-full max-w-sm p-8 bg-base-100 rounded-xl shadow-lg"
                on:submit=register
            >
                <h2 class="text-2xl font-bold text-center mb-2">"VaultChat"</h2>

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
                    <Show when=move || is_pseudo_taken() fallback=|| {}>
                        <div class="alert alert-error p-3 mt-1 rounded-lg shadow-sm">
                            <ul class="list-disc list-inside text-xs font-medium space-y-1">
                                <li>"Pseudo already taken"</li>
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

                <a class="btn btn-soft btn-info w-full" href="/login">
                    "Login"
                </a>
            </form>
        </div>
    }
}
