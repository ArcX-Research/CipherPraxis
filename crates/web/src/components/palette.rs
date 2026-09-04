//! Command-palette style search (Command/Control+K), fully keyboard operable.
use crate::components::badges::StatusBadge;
use crate::state::{use_palette, use_state};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use praxis_core::search::Hit;

#[component]
pub fn SearchPalette() -> impl IntoView {
    let open = use_palette();
    let state = use_state();
    let navigate = use_navigate();
    let query = RwSignal::new(String::new());
    let selected = RwSignal::new(0usize);
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let index = state.index.clone();
    let catalog = state.catalog.clone();
    let hits = Memo::new(move |_| {
        let q = query.get();
        if q.trim().is_empty() {
            Vec::<Hit>::new()
        } else {
            index.query(&q, 12)
        }
    });

    Effect::new(move |_| {
        if open.get() {
            selected.set(0);
            if let Some(el) = input_ref.get() {
                let _ = el.focus();
                el.select();
            }
            if let Some(body) = document().body() {
                let _ = body.class_list().add_1("no-scroll");
            }
        } else if let Some(body) = document().body() {
            let _ = body.class_list().remove_1("no-scroll");
        }
    });

    let go = {
        let navigate = navigate.clone();
        let catalog = catalog.clone();
        move |id: &str| {
            if let Some(e) = catalog.get(id) {
                navigate(&e.route(), NavigateOptions::default());
                open.set(false);
                query.set(String::new());
            }
        }
    };
    let go_enter = go.clone();
    let navigate_all = navigate.clone();

    view! {
        <div class="palette-backdrop" class:open=move || open.get() on:click=move |_| open.set(false)>
            <div
                class="palette glass"
                role="dialog"
                aria-modal="true"
                aria-labelledby="palette-title"
                on:click=move |ev| ev.stop_propagation()
            >
                <div class="palette-input-row">
                    <span class="sr-only" id="palette-title">"Search the knowledge base"</span>
                    <span class="palette-icon" aria-hidden="true" inner_html=crate::components::motif::ICON_SEARCH></span>
                    <input
                        node_ref=input_ref
                        class="palette-input"
                        type="search"
                        placeholder="Search ciphers, methods, tools, or terms…"
                        aria-label="Search query"
                        aria-controls="palette-results"
                        autocomplete="off"
                        prop:value=move || query.get()
                        on:input=move |ev| {
                            query.set(event_target_value(&ev));
                            selected.set(0);
                        }
                        on:keydown=move |ev| {
                            let n = hits.with(|h| h.len());
                            match ev.key().as_str() {
                                "ArrowDown" => {
                                    ev.prevent_default();
                                    if n > 0 { selected.update(|s| *s = (*s + 1) % n); }
                                }
                                "ArrowUp" => {
                                    ev.prevent_default();
                                    if n > 0 { selected.update(|s| *s = (*s + n - 1) % n); }
                                }
                                "Enter" => {
                                    ev.prevent_default();
                                    if let Some(h) = hits.with(|h| h.get(selected.get()).cloned()) {
                                        go_enter(&h.id);
                                    } else if !query.get().trim().is_empty() {
                                        navigate_all(&format!("/find?q={}", js_sys::encode_uri_component(&query.get())), NavigateOptions::default());
                                        open.set(false);
                                    }
                                }
                                _ => {}
                            }
                        }
                    />
                    <kbd class="kbd">"esc"</kbd>
                </div>
                <ul id="palette-results" class="palette-results" role="listbox" aria-label="Results">
                    {move || {
                        let list = hits.get();
                        let q = query.get();
                        if list.is_empty() {
                            if q.trim().is_empty() {
                                view! { <li class="palette-hint mono">"Type to search · use ↑↓ to move · press ↵ to open"</li> }.into_any()
                            } else {
                                view! { <li class="palette-hint">"No entries match "<b>{q}</b>". Try fewer words or a broader term."</li> }.into_any()
                            }
                        } else {
                            list.into_iter().enumerate().map(|(i, h)| {
                                let entry = catalog.get(&h.id).cloned();
                                let go = go.clone();
                                let id = h.id.clone();
                                match entry {
                                    Some(e) => view! {
                                        <li
                                            role="option"
                                            class="palette-hit"
                                            class:selected=move || selected.get() == i
                                            aria-selected=move || (selected.get() == i).to_string()
                                            on:mousemove=move |_| selected.set(i)
                                            on:click=move |_| go(&id)
                                        >
                                            <div class="palette-hit-top">
                                                <span class="mono meta">{format!("{:02} {}", e.section.ordinal(), e.section.short())}</span>
                                                <StatusBadge status=e.status/>
                                            </div>
                                            <div class="palette-hit-title">{e.title.clone()}</div>
                                            <div class="palette-hit-snippet">{h.snippet.clone()}</div>
                                        </li>
                                    }.into_any(),
                                    None => view! { <li></li> }.into_any(),
                                }
                            }).collect_view().into_any()
                        }
                    }}
                </ul>
                <div class="palette-foot mono">
                    <span>{move || format!("{} entries", state.index.len())}</span>
                    <a href=move || format!("/find?q={}", js_sys::encode_uri_component(&query.get())) on:click=move |_| open.set(false)>"See all results →"</a>
                </div>
            </div>
        </div>
    }
}
