//! Root component: router, shell (header, footer, search palette) and route table.
use crate::components::motif::Logo;
use crate::components::palette::SearchPalette;
use crate::pages::{
    entry::EntryPage, find::FindPage, not_found::NotFound, overview::Overview, section::SectionPage,
};
use crate::state::{use_palette, AppState, Palette};
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;
use praxis_core::content::Section;

#[component]
pub fn App() -> impl IntoView {
    provide_context(AppState::load());
    let palette = RwSignal::new(false);
    provide_context(Palette(palette));

    window_event_listener(leptos::ev::keydown, move |ev| {
        let key = ev.key();
        if (ev.meta_key() || ev.ctrl_key()) && (key == "k" || key == "K") {
            ev.prevent_default();
            palette.update(|o| *o = !*o);
        } else if key == "Escape" {
            palette.set(false);
        }
    });

    view! {
        <Router>
            <a class="skip-link" href="#main">"Skip to content"</a>
            <Header/>
            <main id="main" class="main" tabindex="-1">
                <Routes fallback=|| view! { <NotFound/> }>
                    <Route path=path!("/") view=Overview/>
                    <Route path=path!("/find") view=FindPage/>
                    <Route path=path!("/:section") view=SectionPage/>
                    <Route path=path!("/:section/:id") view=EntryPage/>
                </Routes>
            </main>
            <Footer/>
            <SearchPalette/>
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    let palette = use_palette();
    let nav_open = RwSignal::new(false);
    let location = use_location();
    let pathname = location.pathname;
    let is_active = move |slug: &'static str| {
        let p = pathname.get();
        p == format!("/{slug}") || p.starts_with(&format!("/{slug}/"))
    };
    // Close the mobile menu on navigation.
    Effect::new(move |_| {
        pathname.track();
        nav_open.set(false);
    });

    view! {
        <header class="site-header">
            <div class="wrap header-row">
                <a class="brand" href="/" aria-label="Cipher Praxis home">
                    <Logo/>
                    <span class="brand-text">
                        <span class="brand-name">"Cipher Praxis"</span>
                        <span class="brand-sub">"A Dilate Cryptography Knowledge Base"</span>
                    </span>
                </a>
                <nav class="nav" class:open=move || nav_open.get() aria-label="Primary">
                    {Section::ALL
                        .into_iter()
                        .map(|s| {
                            let slug = s.slug();
                            view! {
                                <a
                                    class="nav-link"
                                    href=format!("/{slug}")
                                    aria-current=move || if is_active(slug) { Some("page") } else { None }
                                >
                                    <span class="nav-ord">{format!("{:02}", s.ordinal())}</span>
                                    <span>{s.short()}</span>
                                </a>
                            }
                        })
                        .collect_view()}
                </nav>
                <div class="header-actions">
                    <button
                        type="button"
                        class="search-btn"
                        on:click=move |_| palette.set(true)
                        aria-label="Open search (Command or Control plus K)"
                    >
                        <span class="search-btn-icon" aria-hidden="true" inner_html=crate::components::motif::ICON_SEARCH></span>
                        <span class="search-btn-label">"Search"</span>
                        <kbd class="kbd">"⌘K"</kbd>
                    </button>
                    <button
                        type="button"
                        class="nav-toggle"
                        aria-expanded=move || nav_open.get().to_string()
                        aria-controls="primary-nav"
                        on:click=move |_| nav_open.update(|o| *o = !*o)
                    >
                        <span class="sr-only">"Menu"</span>
                        <span class="nav-toggle-bars" aria-hidden="true"><i></i><i></i></span>
                    </button>
                </div>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let state = crate::state::use_state();
    let generated = state.catalog.generated().to_string();
    let n = state.catalog.len();
    view! {
        <footer class="site-footer">
            <div class="wrap footer-grid">
                <div class="footer-brand">
                    <Logo/>
                    <div>
                        <div class="brand-name">"Cipher Praxis"</div>
                        <div class="footer-tag">"A Dilate Cryptography Knowledge Base — theory made operational through equations, attacks, controls and WebAssembly labs."</div>
                    </div>
                </div>
                <div class="footer-cols">
                    <div>
                        <div class="footer-h">"Browse"</div>
                        {Section::ALL.iter().take(6).map(|s| view! { <a href=format!("/{}", s.slug())>{s.title()}</a> }).collect_view()}
                    </div>
                    <div>
                        <div class="footer-h">"More"</div>
                        {Section::ALL.iter().skip(6).map(|s| view! { <a href=format!("/{}", s.slug())>{s.title()}</a> }).collect_view()}
                        <a href="/find">"Search"</a>
                    </div>
                    <div>
                        <div class="footer-h">"Build"</div>
                        <div class="footer-meta mono">
                            <div>{format!("{n} entries")}</div>
                            <div>{format!("bundle {generated}")}</div>
                            <div>"Rust · Leptos · WebAssembly"</div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="wrap footer-line mono">
                <span>"© Dilate Technologies · Cipher Praxis"</span>
                <span>"Light theme · no tracking · runs entirely in your browser"</span>
            </div>
        </footer>
    }
}
