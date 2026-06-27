use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::components::navbar::nav::Navbar;
use crate::pages::chat::chat::Chat;
use crate::pages::home::home::Home;
use crate::pages::{login::login::Login, register::register::Register};
use crate::store::auth::{get_current_user, provide_auth_state};

#[component]
pub fn App() -> impl IntoView {
    provide_auth_state();

    view! {
        <Router>
            <div class="h-[100dvh] w-full flex flex-col overflow-hidden bg-base-200">
                <Navbar />

                <main class="flex-1 min-h-0 overflow-hidden relative">
                    // The Router looks at the URL and renders only the matching Route
                    <Routes fallback=|| view! { <h1>"404 Not Found"</h1> }>
                        <Route
                            path=path!("/")
                            view=Home
                        />
                        <ProtectedRoute
                            path=path!("/login")
                            redirect_path=|| "/chat"
                            condition=move || Some(!get_current_user().get().is_some())
                            view=Login
                        />
                        <ProtectedRoute
                            path=path!("/register")
                            redirect_path=|| "/chat"
                            condition=move || Some(!get_current_user().get().is_some())
                            view=Register
                        />
                        <ProtectedRoute
                            path=path!("/chat")
                            redirect_path=|| "/login"
                            condition=move || Some(get_current_user().get().is_some())
                            view=Chat
                        />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
