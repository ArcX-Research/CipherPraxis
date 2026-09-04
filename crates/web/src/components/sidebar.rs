//! Persistent knowledge index for the database-style application shell.
use crate::components::motif::ICON_SEARCH;
use crate::state::use_state;
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use praxis_core::content::{Entry, Section};
use praxis_core::search::tokenize;

#[component]
pub fn KnowledgeSidebar(open: RwSignal<bool>) -> impl IntoView {
    let state = use_state();
    let catalog = state.catalog;
    let query = RwSignal::new(String::new());
    let pathname = use_location().pathname;

    Effect::new(move |_| {
        pathname.track();
        open.set(false);
    });

    let sections: Vec<(Section, Vec<Entry>)> = Section::ALL
        .into_iter()
        .map(|section| {
            (
                section,
                catalog.section(section).into_iter().cloned().collect(),
            )
        })
        .collect();

    view! {
        <button
            type="button"
            class="kb-sidebar-backdrop"
            class:open=move || open.get()
            aria-label="Close site menu"
            on:click=move |_| open.set(false)
        ></button>
        <aside
            id="knowledge-sidebar"
            class="kb-sidebar"
            class:open=move || open.get()
            aria-label="Site menu"
        >
            <div class="kb-sidebar-head">
                <div>
                    <p class="eyebrow mono">"Browse"</p>
                    <p class="kb-sidebar-count mono">{format!("{} entries · {} sections", catalog.len(), Section::ALL.len())}</p>
                </div>
                <button
                    type="button"
                    class="kb-sidebar-close"
                    aria-label="Close site menu"
                    on:click=move |_| open.set(false)
                >"×"</button>
            </div>

            <label class="kb-sidebar-search">
                <span class="sr-only">"Search the site menu"</span>
                <span class="filter-icon" aria-hidden="true" inner_html=ICON_SEARCH></span>
                <input
                    type="search"
                    placeholder=format!("Search {} entries…", catalog.len())
                    autocomplete="off"
                    prop:value=move || query.get()
                    on:input=move |ev| query.set(event_target_value(&ev))
                />
                <button
                    type="button"
                    class="kb-sidebar-clear"
                    hidden=move || query.get().is_empty()
                    aria-label="Clear filter"
                    on:click=move |_| query.set(String::new())
                >"×"</button>
            </label>

            <nav class="kb-tree" aria-label="Browse all entries">
                <a
                    class="kb-home-link"
                    href="/"
                    aria-current=move || (pathname.get() == "/").then_some("page")
                >
                    <span class="kb-node-mark" aria-hidden="true"></span>
                    <span>"Overview"</span>
                    <span class="mono kb-node-meta">"HOME"</span>
                </a>
                {sections
                    .into_iter()
                    .map(|(section, entries)| {
                        view! { <SidebarSection section=section entries=entries query=query/> }
                    })
                    .collect_view()}
            </nav>
            <div class="kb-sidebar-foot mono">
                <span class="kb-live-dot" aria-hidden="true"></span>
                <span>"Browser search ready"</span>
            </div>
        </aside>
    }
}

#[component]
fn SidebarSection(section: Section, entries: Vec<Entry>, query: RwSignal<String>) -> impl IntoView {
    let pathname = use_location().pathname;
    let section_route = format!("/{}", section.slug());
    let active_prefix = format!("{section_route}/");
    let active = Memo::new(move |_| {
        let current = pathname.get();
        current == section_route || current.starts_with(&active_prefix)
    });
    let expanded = RwSignal::new(active.get_untracked());
    Effect::new(move |_| expanded.set(active.get()));
    let entries_for_filter = entries.clone();
    let visible = Memo::new(move |_| {
        let terms = tokenize(&query.get());
        entries_for_filter
            .iter()
            .filter(|entry| {
                if terms.is_empty() {
                    return true;
                }
                let haystack = tokenize(&format!(
                    "{} {} {} {} {}",
                    entry.title,
                    entry.subtitle.clone().unwrap_or_default(),
                    entry.family_or_default(),
                    entry.summary,
                    entry.tags.join(" ")
                ));
                terms
                    .iter()
                    .all(|term| haystack.iter().any(|word| word.starts_with(term)))
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    view! {
        <section class="kb-tree-group" hidden=move || visible.with(Vec::is_empty)>
            <div
                class="kb-tree-heading"
                class:active=move || active.get()
            >
                <button
                    type="button"
                    class="kb-tree-toggle"
                    aria-label=format!("Show or hide {} entries", section.short())
                    aria-expanded=move || expanded.get().to_string()
                    on:click=move |_| expanded.update(|value| *value = !*value)
                >
                    <span class="kb-tree-caret" class:open=move || expanded.get() aria-hidden="true"></span>
                </button>
                <a
                    class="kb-tree-section-link"
                    href=format!("/{}", section.slug())
                    aria-current=move || active.get().then_some("location")
                >
                    <span class="mono kb-tree-ord">{format!("{:02}", section.ordinal())}</span>
                    <span>{section.short()}</span>
                </a>
                <span class="mono kb-tree-count">{move || visible.with(Vec::len)}</span>
            </div>
            <div class="kb-tree-items" hidden=move || query.get().trim().is_empty() && !expanded.get()>
                {move || {
                    visible
                        .get()
                        .into_iter()
                        .map(|entry| {
                            let route = entry.route();
                            let route_for_current = route.clone();
                            let status = entry.status.slug();
                            view! {
                                <a
                                    class="kb-tree-item"
                                    href=route
                                    aria-current=move || (pathname.get() == route_for_current).then_some("page")
                                    title=entry.title.clone()
                                >
                                    <span class=format!("kb-status-dot status-dot-{status}") aria-hidden="true"></span>
                                    <span class="kb-tree-title">{entry.title.clone()}</span>
                                </a>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </section>
    }
}
