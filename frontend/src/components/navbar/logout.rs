use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::store::auth::set_current_user;

#[component]
pub fn LogoutBtn() -> impl IntoView {
    let navigate = use_navigate();
    let logout = move |_| {
        set_current_user(None);
        navigate("/", Default::default());
    };

    view! {
        <button class="btn btn-error btn-sm" on:click=logout>"Logout"</button>
    }
}
