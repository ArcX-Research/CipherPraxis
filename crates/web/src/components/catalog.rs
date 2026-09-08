//! Shared tools rail for category indexes and full search.
use leptos::prelude::*;

#[component]
pub fn CatalogTools(title: &'static str, children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <aside class="catalog-tools" class:is-open=move || open.get() aria-label=title>
            <button type="button" class="catalog-tools-toggle" aria-expanded=move || open.get().to_string() aria-controls="catalog-tools-body" on:click=move |_| open.update(|value| *value = !*value)>
                <span>{title}</span><span class="mono" aria-hidden="true">{move || if open.get() { "−" } else { "+" }}</span>
            </button>
            <div id="catalog-tools-body" class="catalog-tools-body">
                <h2 class="catalog-tools-title">{title}</h2>
                {children()}
            </div>
        </aside>
    }
}
