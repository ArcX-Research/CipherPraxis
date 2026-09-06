//! Full search results page (`/find?q=`), shareable counterpart of the palette.
use crate::components::cards::EntryCard;
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
        <section class="wrap page">
            <p class="eyebrow mono">"Search"</p>
            <h1 class="display">"Search the knowledge base"</h1>
            <form class="find-form" role="search" on:submit=move |ev| ev.prevent_default()>
                <label class="sr-only" for="find-input">"Search query"</label>
                <input
                    id="find-input"
                    class="find-input"
                    type="search"
                    placeholder="Try coincidence, annealing, Quagmire, or planted control"
                    prop:value=move || local.get()
                    on:input=move |ev| local.set(event_target_value(&ev))
                />
            </form>
            <p class="mono meta">{move || {
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
            <div class="card-grid">
                {move || hits.get().into_iter().filter_map(|h| catalog.get(&h.id).cloned()).map(|e| view! { <EntryCard entry=e/> }).collect_view()}
            </div>
        </section>
    }
}
