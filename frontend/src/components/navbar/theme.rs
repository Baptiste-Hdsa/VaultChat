use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_use::{ColorMode, UseColorModeOptions, UseColorModeReturn, use_color_mode_with_options};

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .attribute("data-theme")
            .storage_key("vaultchat-theme"),
    );

    let toggle_theme = move |_| {
        if mode.get() == ColorMode::Dark {
            set_mode.set(ColorMode::Light);
        } else {
            set_mode.set(ColorMode::Dark);
        }
    };

    view! {
        <button class="btn btn-circle btn-ghost text-xl" on:click=toggle_theme>
            {move || if mode.get() == ColorMode::Dark { view!{<Icon icon=icondata::BiSunSolid />} } else { view!{<Icon icon=icondata::BiMoonSolid />} }}
        </button>
    }
}
