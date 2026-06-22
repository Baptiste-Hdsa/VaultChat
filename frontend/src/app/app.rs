use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::components::navbar::nav::Navbar;
use crate::pages::home::home::Home;
use crate::pages::{login::login::Login, register::register::Register};
use crate::store::auth::provide_auth_state;

#[component]
pub fn App() -> impl IntoView {
    provide_auth_state();

    view! {
        <Router>
            <Navbar />

            <main class="p-4">
                // The Router looks at the URL and renders only the matching Route
                <Routes fallback=|| view! { <h1>"404 Not Found"</h1> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/login") view=Login />
                    <Route path=path!("/register") view=Register />
                </Routes>
            </main>
        </Router>
    }
}
