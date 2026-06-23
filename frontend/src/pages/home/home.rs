// pages/home.rs
use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-base-200 flex flex-col text-base-content">

            // --- HERO SECTION ---
            <main class="flex-grow flex flex-col items-center justify-center pt-8 pb-16">
                <div class="hero bg-base-200 py-12">
                    <div class="hero-content text-center">
                        <div class="max-w-2xl">
                            <h1 class="text-5xl font-bold tracking-tight mb-6">
                                "Your Data. Your Chat."
                            </h1>
                            <p class="text-xl mb-8 opacity-80">
                                "A lightning-fast, self-hosted messaging platform built with Rust. No tracking, zero external CDN calls, just pure federated communication."
                            </p>
                            <div class="flex flex-col sm:flex-row gap-4 justify-center">
                                <A href="/register" attr:class="btn btn-primary btn-lg gap-2">
                                    <Icon icon=i::LuUserPlus/>
                                    "Create Account"
                                </A>
                                <A href="/login" attr:class="btn btn-outline btn-lg gap-2">
                                    <Icon icon=i::LuLogIn/>
                                    "Login to Instance"
                                </A>
                            </div>
                        </div>
                    </div>
                </div>

                // --- FEATURES GRID ---
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-6xl mx-auto px-6 w-full">

                    // Feature 1
                    <div class="card bg-base-100 shadow-xl border border-base-300">
                        <div class="card-body items-center text-center">
                            <div class="p-4 bg-success/10 rounded-full mb-2">
                                <Icon icon=i::LuShieldCheck/>
                            </div>
                            <h2 class="card-title">"Zero-Leak Security"</h2>
                            <p class="opacity-75">
                                "Compiled to a static WebAssembly bundle. Your interface never makes unauthorized calls to the outside world."
                            </p>
                        </div>
                    </div>

                    // Feature 2
                    <div class="card bg-base-100 shadow-xl border border-base-300">
                        <div class="card-body items-center text-center">
                            <div class="p-4 bg-warning/10 rounded-full mb-2">
                                <Icon icon=i::LuZap/>
                            </div>
                            <h2 class="card-title">"Blazing Fast"</h2>
                            <p class="opacity-75">
                                "Client-side rendering ensures instantaneous navigation between chat rooms without constant server round-trips."
                            </p>
                        </div>
                    </div>

                    // Feature 3
                    <div class="card bg-base-100 shadow-xl border border-base-300">
                        <div class="card-body items-center text-center">
                            <div class="p-4 bg-info/10 rounded-full mb-2">
                                <Icon icon=i::LuServer/>
                            </div>
                            <h2 class="card-title">"Fully Self-Hosted"</h2>
                            <p class="opacity-75">
                                "Deploy anywhere with Docker. Maintain absolute ownership of your PostgreSQL database and message history."
                            </p>
                        </div>
                    </div>

                </div>
            </main>

            // --- FOOTER ---
            <footer class="footer footer-center p-6 bg-base-300 text-base-content mt-auto">
                <aside>
                    <p class="opacity-60">"© 2026 VaultChat Server Instance"</p>
                </aside>
            </footer>

        </div>
    }
}
