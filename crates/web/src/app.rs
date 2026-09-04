//! Root component: router, shell (header, footer, search palette) and route table.
use crate::components::motif::Logo;
use crate::components::palette::SearchPalette;
use crate::components::sidebar::KnowledgeSidebar;
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
    let sidebar = RwSignal::new(false);
    provide_context(Palette(palette));

    window_event_listener(leptos::ev::keydown, move |ev| {
        let key = ev.key();
        if (ev.meta_key() || ev.ctrl_key()) && (key == "k" || key == "K") {
            ev.prevent_default();
            palette.update(|o| *o = !*o);
        } else if key == "Escape" {
            palette.set(false);
            sidebar.set(false);
        }
    });

    view! {
        <Router>
            <a class="skip-link" href="#main">"Skip to content"</a>
            <Header sidebar=sidebar/>
            <div class="kb-shell">
                <KnowledgeSidebar open=sidebar/>
                <div class="kb-workspace">
                    <main id="main" class="main" tabindex="-1">
                        <Routes fallback=|| view! { <NotFound/> }>
                            <Route path=path!("/") view=Overview/>
                            <Route path=path!("/find") view=FindPage/>
                            <Route path=path!("/:section") view=SectionPage/>
                            <Route path=path!("/:section/:id") view=EntryPage/>
                        </Routes>
                    </main>
                    <Footer/>
                </div>
            </div>
            <SearchPalette/>
            <HashScroller/>
        </Router>
    }
}

#[component]
fn Header(sidebar: RwSignal<bool>) -> impl IntoView {
    let palette = use_palette();
    let state = crate::state::use_state();
    let total = state.catalog.len();

    view! {
        <header class="site-header">
            <div class="header-row">
                <a class="brand" href="/" aria-label="Cipher Praxis home">
                    <Logo/>
                    <span class="brand-text">
                        <span class="brand-name">"Cipher Praxis"</span>
                        <span class="brand-sub">"Cryptography knowledge base"</span>
                    </span>
                </a>
                <div class="header-context mono">
                    <span class="kb-live-dot" aria-hidden="true"></span>
                    <span>{format!("{total} entries")}</span>
                    <span class="header-context-sep">"/"</span>
                    <span>"runs in your browser"</span>
                </div>
                <div class="header-actions">
                    <button
                        type="button"
                        class="search-btn"
                        on:click=move |_| palette.set(true)
                        aria-label="Open search (Command or Control and K)"
                    >
                        <span class="search-btn-icon" aria-hidden="true" inner_html=crate::components::motif::ICON_SEARCH></span>
                        <span class="search-btn-label">"Search"</span>
                        <kbd class="kbd">"⌘K"</kbd>
                    </button>
                    <button
                        type="button"
                        class="nav-toggle"
                        aria-expanded=move || sidebar.get().to_string()
                        aria-controls="knowledge-sidebar"
                        aria-label="Open site menu"
                        on:click=move |_| sidebar.update(|open| *open = !*open)
                    >
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
                        <div class="footer-tag">"Learn how ciphers work, how attacks test them, and how evidence supports each result."</div>
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
                <span>"No tracking · runs in your browser"</span>
            </div>
        </footer>
    }
}

/// Scrolls to the element named by the URL fragment after every navigation, retrying briefly
/// while the routed page is still rendering. Client-side rendering needs this because the
/// browser only scrolls to fragments on full page loads.
#[component]
fn HashScroller() -> impl IntoView {
    let location = use_location();
    let hash = location.hash;
    let pathname = location.pathname;
    Effect::new(move |_| {
        pathname.track();
        let h = hash.get();
        let id = h.trim_start_matches('#').to_string();
        if id.is_empty() {
            return;
        }
        fn attempt(id: String, tries_left: u32) {
            match document().get_element_by_id(&id) {
                Some(el) => el.scroll_into_view(),
                None if tries_left > 0 => set_timeout(
                    move || attempt(id, tries_left - 1),
                    std::time::Duration::from_millis(80),
                ),
                None => {}
            }
        }
        attempt(id, 12);
    });
    view! { <></> }
}
