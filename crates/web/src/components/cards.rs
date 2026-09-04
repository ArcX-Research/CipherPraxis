//! Entry cards used on section indexes, related lists and search results.
use crate::components::badges::StatusBadge;
use leptos::prelude::*;
use praxis_core::content::Entry;

/// `show_section` adds the section name to the meta line (for cross-section lists such as the
/// overview and search results); `show_family` adds the family (off when the surrounding list is
/// already grouped by family, so the card never repeats its heading).
#[component]
pub fn EntryCard(
    entry: Entry,
    #[prop(optional)] compact: bool,
    #[prop(default = true)] show_section: bool,
    #[prop(default = true)] show_family: bool,
) -> impl IntoView {
    let route = entry.route();
    let family = entry.family_or_default().to_string();
    let section = entry.section;
    let tags = entry.tags.clone();
    let meta: Vec<&str> = [
        show_section.then(|| section.short()),
        show_family.then_some(family.as_str()),
    ]
    .into_iter()
    .flatten()
    .collect();
    let meta = meta.join(" · ");
    let has_meta = !meta.is_empty();
    view! {
        <a class="card" class:card-compact=compact href=route>
            <div class="card-top" class:card-top-end=!has_meta>
                {has_meta.then(|| view! { <span class="mono meta">{meta.clone()}</span> })}
                <StatusBadge status=entry.status/>
            </div>
            <h3 class="card-title">{entry.title.clone()}</h3>
            {entry.subtitle.clone().map(|s| view! { <p class="card-sub">{s}</p> })}
            {(!compact).then(|| view! { <p class="card-summary">{praxis_core::render::markdown_to_text(&entry.summary)}</p> })}
            {(!compact && !tags.is_empty()).then(|| view! {
                <div class="card-tags">
                    {tags.iter().take(4).map(|t| view! { <span class="tag">{t.clone()}</span> }).collect_view()}
                </div>
            })}
        </a>
    }
}

/// A list of entry rows; entries beyond `limit` sit behind a native disclosure so long
/// "cited by" lists do not push the evidence panel off the rail.
#[component]
pub fn RowList(entries: Vec<Entry>, #[prop(default = 8)] limit: usize) -> impl IntoView {
    let total = entries.len();
    let (head, tail): (Vec<Entry>, Vec<Entry>) = if total > limit + 2 {
        let mut e = entries;
        let tail = e.split_off(limit);
        (e, tail)
    } else {
        (entries, Vec::new())
    };
    let more = tail.len();
    view! {
        <div class="row-list">
            {head.into_iter().map(|e| view! { <EntryRow entry=e/> }).collect_view()}
            {(more > 0).then(|| view! {
                <details class="row-more">
                    <summary>{format!("Show {more} more")}</summary>
                    <div class="row-list">{tail.into_iter().map(|e| view! { <EntryRow entry=e/> }).collect_view()}</div>
                </details>
            })}
        </div>
    }
}

#[component]
pub fn EntryRow(entry: Entry) -> impl IntoView {
    let route = entry.route();
    view! {
        <a class="row-link" href=route>
            <span class="row-title">{entry.title.clone()}</span>
            <span class="row-meta mono">{entry.section.short()}</span>
            <StatusBadge status=entry.status/>
        </a>
    }
}
