//! Full search results page (`/find?q=`), shareable counterpart of the palette.
use crate::components::cards::EntryCard;
use crate::components::catalog::CatalogTools;
use crate::state::use_state;
use crate::util::set_title;
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

#[component]
pub fn FindPage() -> impl IntoView {
    set_title("Search");
    let state = use_state();
    let query_map = use_query_map();
    let q = Memo::new(move |_| query_map.with(|m| m.get("q").unwrap_or_default()));
    let local = RwSignal::new(String::new());
    Effect::new(move |_| local.set(q.get()));
    let index = state.index.clone();
    let catalog = state.catalog.clone();
    let hits = Memo::new(move |_| {
        let query = local.get();
        if query.trim().is_empty() {
            Vec::new()
        } else {
            index.query(&query, 60)
        }
    });

    view! {
        <section class="wrap page catalog-layout">
            <header class="section-header">
                <h1 class="display">"Search the knowledge base"</h1>
                <p class="lede">"Find cipher systems, mathematical foundations, and methods across every category."</p>
            </header>
            <CatalogTools title="Search">
                <form class="filter-block" role="search" on:submit=move |ev| ev.prevent_default()>
                    <label class="filter-label" for="find-input">"Search query"</label>
                    <div class="filter-search">
                        <span class="filter-icon" aria-hidden="true" inner_html=crate::components::motif::ICON_SEARCH></span>
                        <input
                            id="find-input"
                            type="search"
                            placeholder="Title, topic, or keyword…"
                            prop:value=move || local.get()
                            on:input=move |ev| local.set(event_target_value(&ev))
                        />
                    </div>
                </form>
                <p class="filter-help">"Try coincidence, annealing, Quagmire, or planted control."</p>
                <button type="button" class="filter-reset" disabled=move || local.get().is_empty() on:click=move |_| local.set(String::new())>"Clear search"</button>
            </CatalogTools>
            <div class="catalog-results">
            <p class="sr-only" role="status">{move || {
                let n = hits.with(|h| h.len());
                let q = local.get();
                if q.trim().is_empty() { "Enter a word or phrase to search every section.".to_string() } else { format!("{n} result{} for “{q}”", if n == 1 { "" } else { "s" }) }
            }}</p>
            {move || {
                let has_query = !local.get().trim().is_empty();
                (has_query && hits.with(|h| h.is_empty())).then(|| view! {
                    <p class="empty">"No entries match that search. Try fewer words or a broader term."</p>
                })
            }}
            <div class="catalog-list">
                {move || hits.get().into_iter().filter_map(|h| catalog.get(&h.id).cloned()).map(|e| view! { <EntryCard entry=e/> }).collect_view()}
            </div>
            </div>
        </section>
    }
}
