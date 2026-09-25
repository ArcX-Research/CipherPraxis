//! Category indexes share one results layout and filter rail.
use crate::components::badges::StatusBadge;
use crate::components::cards::EntryCard;
use crate::components::catalog::CatalogTools;
use crate::pages::not_found::NotFound;
use crate::state::{use_state, CORPUS_ROOT};
use crate::util::set_title;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use praxis_core::content::{Entry, Section, Status};
use praxis_core::search::tokenize;
use std::collections::{BTreeMap, BTreeSet};

#[component]
pub fn SectionPage() -> impl IntoView {
    let params = use_params_map();
    let slug = Memo::new(move |_| params.with(|p| p.get("section").unwrap_or_default()));
    view! {
        {move || match Section::from_slug(&slug.get()) {
            Some(section) => view! { <SectionIndex section=section/> }.into_any(),
            None => view! { <NotFound/> }.into_any(),
        }}
    }
}

#[component]
fn SectionHeader(section: Section) -> impl IntoView {
    view! {
        <header class="section-header">
            <h1 class="display">{section.title()}</h1>
            <p class="lede">{section.blurb()}</p>
        </header>
    }
}

#[component]
fn SectionIndex(section: Section) -> impl IntoView {
    set_title(section.title());
    let state = use_state();
    let catalog = state.catalog.clone();
    let mut entries: Vec<Entry> = catalog.section(section).into_iter().cloned().collect();
    if section == Section::Glossary {
        entries.sort_by_key(|entry| entry.title.to_lowercase());
    }
    let families = catalog.families(section);
    let count = entries.len();
    let can_group = !matches!(section, Section::Glossary | Section::References);
    let mut topics: BTreeMap<String, (String, usize)> = BTreeMap::new();
    for entry in &entries {
        let mut seen = BTreeSet::new();
        for tag in &entry.tags {
            let key = tag.to_lowercase();
            if seen.insert(key.clone()) {
                topics
                    .entry(key)
                    .and_modify(|(_, count)| *count += 1)
                    .or_insert((tag.clone(), 1));
            }
        }
    }
    let status_counts: Vec<(Status, usize)> = Status::ALL
        .iter()
        .map(|status| {
            (
                *status,
                entries
                    .iter()
                    .filter(|entry| entry.status == *status)
                    .count(),
            )
        })
        .filter(|(_, count)| *count > 0)
        .collect();

    let status_filter = RwSignal::new(Option::<Status>::None);
    let family_filter = RwSignal::new(String::new());
    let topic_filter = RwSignal::new(String::new());
    let text_filter = RwSignal::new(String::new());
    let group_by_family = RwSignal::new(true);
    let has_filters = Memo::new(move |_| {
        status_filter.get().is_some()
            || !family_filter.get().is_empty()
            || !topic_filter.get().is_empty()
            || !text_filter.get().is_empty()
    });

    // Tokenize each entry once, including the bibliography on the reference index.
    let searchable: Vec<_> = entries
        .into_iter()
        .map(|entry| {
            let references = if section == Section::References {
                entry
                    .references
                    .iter()
                    .map(|reference| {
                        format!(
                            "{} {} {} {}",
                            reference.title,
                            reference.author.as_deref().unwrap_or_default(),
                            reference
                                .year
                                .map(|year| year.to_string())
                                .unwrap_or_default(),
                            reference.note.as_deref().unwrap_or_default()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            } else {
                String::new()
            };
            let words = tokenize(&format!(
                "{} {} {} {} {} {}",
                entry.title,
                entry.subtitle.as_deref().unwrap_or_default(),
                entry.summary,
                entry.family_or_default(),
                entry.tags.join(" "),
                references
            ));
            (entry, words)
        })
        .collect();
    let filtered = Memo::new(move |_| {
        let status = status_filter.get();
        let family = family_filter.get();
        let topic = topic_filter.get();
        let terms = tokenize(&text_filter.get());
        searchable
            .iter()
            .filter(|(entry, _)| status.is_none_or(|value| entry.status == value))
            .filter(|(entry, _)| family.is_empty() || entry.family_or_default() == family)
            .filter(|(entry, _)| {
                topic.is_empty() || entry.tags.iter().any(|tag| tag.to_lowercase() == topic)
            })
            .filter(|(_, words)| {
                terms
                    .iter()
                    .all(|term| words.iter().any(|word| word.starts_with(term)))
            })
            .map(|(entry, _)| entry.clone())
            .collect::<Vec<_>>()
    });
    let letters = Memo::new(move |_| {
        filtered.with(|entries| {
            entries
                .iter()
                .filter_map(|entry| entry.title.chars().next())
                .map(|letter| letter.to_ascii_uppercase())
                .collect::<BTreeSet<_>>()
        })
    });

    view! {
        <section class="wrap page catalog-layout">
            <SectionHeader section=section/>
            <CatalogTools title="Search and filters">
                <div class="filter-block">
                    <label class="filter-label" for="category-search">"Search this category"</label>
                    <div class="filter-search">
                        <span class="filter-icon" aria-hidden="true" inner_html=crate::components::motif::ICON_SEARCH></span>
                        <input id="category-search" type="search" placeholder="Title, topic, or keyword…" prop:value=move || text_filter.get() on:input=move |ev| text_filter.set(event_target_value(&ev))/>
                    </div>
                </div>
                <div class="filter-block">
                    <label class="filter-select">
                        <span class="filter-label">"Family"</span>
                        <select prop:value=move || family_filter.get() on:change=move |ev| family_filter.set(event_target_value(&ev))>
                            <option value="">"All families"</option>
                            {families.iter().map(|family| view! { <option value=family.clone()>{family.clone()}</option> }).collect_view()}
                        </select>
                    </label>
                    {can_group.then(|| view! {
                        <label class="filter-toggle">
                            <input type="checkbox" prop:checked=move || group_by_family.get() on:change=move |ev| group_by_family.set(event_target_checked(&ev))/>
                            <span>"Group by family"</span>
                        </label>
                    })}
                </div>
                {(!topics.is_empty()).then(|| view! {
                    <div class="filter-block">
                        <label class="filter-select">
                            <span class="filter-label">"Topic"</span>
                            <select prop:value=move || topic_filter.get() on:change=move |ev| topic_filter.set(event_target_value(&ev))>
                                <option value="">"All topics"</option>
                                {topics.iter().map(|(key, (label, count))| view! { <option value=key.clone()>{format!("{label} ({count})")}</option> }).collect_view()}
                            </select>
                        </label>
                    </div>
                })}
                <fieldset class="filter-block filter-statuses">
                    <legend class="filter-label">"Evidence status"</legend>
                    <button type="button" class="filter-status" class:active=move || status_filter.get().is_none() aria-pressed=move || status_filter.get().is_none().to_string() on:click=move |_| status_filter.set(None)>
                        <span>"All statuses"</span><span class="mono meta">{count}</span>
                    </button>
                    {status_counts.into_iter().map(|(status, count)| view! {
                        <button type="button" class="filter-status" class:active=move || status_filter.get() == Some(status) aria-pressed=move || (status_filter.get() == Some(status)).to_string() on:click=move |_| status_filter.set(if status_filter.get() == Some(status) { None } else { Some(status) })>
                            <span class="filter-status-name"><span class=format!("kb-status-dot status-dot-{}", status.slug()) aria-hidden="true"></span>{status.label()}</span><span class="mono meta">{count}</span>
                        </button>
                    }).collect_view()}
                </fieldset>
                <button type="button" class="filter-reset" disabled=move || !has_filters.get() on:click=move |_| {
                    status_filter.set(None);
                    family_filter.set(String::new());
                    topic_filter.set(String::new());
                    text_filter.set(String::new());
                }>"Reset filters"</button>
                {(section == Section::Glossary).then(|| view! {
                    <div class="filter-block">
                        <p class="filter-label">"Jump to letter"</p>
                        <nav class="letter-nav mono" aria-label="Jump to letter">
                            {move || letters.get().into_iter().map(|letter| view! { <a href=format!("#g-{letter}")>{letter.to_string()}</a> }).collect_view()}
                        </nav>
                    </div>
                })}
            </CatalogTools>
            <div class="catalog-results">
                <p class="sr-only" role="status">{move || format!("{} matching entries", filtered.with(|entries| entries.len()))}</p>
                {move || filtered.with(|entries| entries.is_empty()).then(|| view! {
                    <p class="empty">"No entries match. Reset the filters or try a broader search."</p>
                })}
                {match section {
                    Section::Glossary => view! { <GlossaryList entries=filtered/> }.into_any(),
                    Section::References => view! { <ReferenceList entries=filtered/><ReferenceSources/> }.into_any(),
                    _ => view! {
                        {move || {
                            let list = filtered.get();
                            if group_by_family.get() {
                                let mut groups: BTreeMap<String, Vec<Entry>> = BTreeMap::new();
                                for entry in list {
                                    groups.entry(entry.family_or_default().to_string()).or_default().push(entry);
                                }
                                groups.into_iter().map(|(family, entries)| view! {
                                    <div class="family-group">
                                        <h2 class="family-h">{family}</h2>
                                        <div class="catalog-list">{entries.into_iter().map(|entry| view! { <EntryCard entry=entry show_section=false show_family=false/> }).collect_view()}</div>
                                    </div>
                                }).collect_view().into_any()
                            } else {
                                view! { <div class="catalog-list">{list.into_iter().map(|entry| view! { <EntryCard entry=entry show_section=false/> }).collect_view()}</div> }.into_any()
                            }
                        }}
                    }.into_any(),
                }}
            </div>
        </section>
    }
}

#[component]
fn GlossaryList(entries: Memo<Vec<Entry>>) -> impl IntoView {
    let catalog = use_state().catalog.clone();
    view! {
        <dl class="glossary">
            {move || {
                let mut last = ' ';
                entries.get().into_iter().map(|entry| {
                    let first = entry.title.chars().next().map(|letter| letter.to_ascii_uppercase()).unwrap_or('#');
                    let anchor = if first != last { last = first; Some(first) } else { None };
                    let related: Vec<Entry> = catalog.related(&entry).into_iter().cloned().collect();
                    view! {
                        {anchor.map(|letter| view! { <div class="glossary-letter mono" id=format!("g-{letter}")>{letter.to_string()}</div> })}
                        <div class="glossary-item" id=entry.id.clone()>
                            <dt><a href=entry.route()>{entry.title.clone()}</a>{entry.family.clone().map(|family| view! { <span class="mono meta">{family}</span> })}</dt>
                            <dd>
                                <div inner_html=praxis_core::render::markdown_to_html(&entry.summary)></div>
                                {(!related.is_empty()).then(|| view! {
                                    <p class="glossary-related">"See "{related.iter().enumerate().map(|(index, related)| view! { {(index > 0).then_some(", ")}<a href=related.route()>{related.title.clone()}</a> }).collect_view()}"."</p>
                                })}
                            </dd>
                        </div>
                    }
                }).collect_view()
            }}
        </dl>
    }
}

#[component]
fn ReferenceList(entries: Memo<Vec<Entry>>) -> impl IntoView {
    view! {
        <div class="catalog-references">
            {move || entries.get().into_iter().map(|entry| {
                let references = entry.references.clone();
                view! {
                    <article class="ref-group" id=entry.id.clone()>
                        <div class="card-top"><span class="mono meta">{entry.family_or_default().to_string()}</span><StatusBadge status=entry.status/></div>
                        <h2 class="ref-group-title"><a href=entry.route()>{entry.title.clone()}</a></h2>
                        <p class="card-summary">{praxis_core::render::markdown_to_text(&entry.summary)}</p>
                        {(!references.is_empty()).then(|| view! {
                            <ol class="ref-list">
                                {references.into_iter().map(|reference| {
                                    let meta = [reference.author.clone(), reference.year.map(|year| year.to_string())].into_iter().flatten().collect::<Vec<_>>().join(", ");
                                    view! {
                                        <li>
                                            {match reference.url.clone() { Some(url) => view! { <a href=url target="_blank" rel="noopener">{reference.title.clone()}</a> }.into_any(), None => view! { <span class="ref-title">{reference.title.clone()}</span> }.into_any() }}
                                            {(!meta.is_empty()).then(|| view! { <span class="ref-meta">{format!(" — {meta}")}</span> })}
                                            {reference.note.clone().map(|note| view! { <div class="prov-note">{note}</div> })}
                                        </li>
                                    }
                                }).collect_view()}
                            </ol>
                        })}
                    </article>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
fn ReferenceSources() -> impl IntoView {
    let catalog = use_state().catalog.clone();
    let prov_rows: Vec<(String, Vec<Entry>)> = catalog
        .provenance_map()
        .into_iter()
        .map(|(path, entries)| (path, entries.into_iter().cloned().collect()))
        .collect();
    let prov_count = prov_rows.len();
    view! {
        <section class="catalog-sources">
            <div class="section-head">
                <p class="eyebrow mono">"Source files"</p>
                <h2 class="display-sm">{format!("{prov_count} source files support entries in this knowledge base")}</h2>
                <p class="lede-sm">"Archival identifiers trace individual claims to their records. Unprefixed paths refer to "<code>{CORPUS_ROOT}</code>"; archive prefixes identify additional source collections."</p>
            </div>
            <details class="evidence-records">
                <summary>"Browse the archival source index"</summary>
            <div class="table-scroll">
                <table class="prov-table">
                    <thead><tr><th>"Archival record"</th><th>"Cited by"</th></tr></thead>
                    <tbody>
                        {prov_rows.into_iter().map(|(p, es)| view! {
                            <tr>
                                <td><code>{p}</code></td>
                                <td>{es.iter().enumerate().map(|(i, e)| view! { {(i > 0).then_some(", ")}<a href=e.route()>{e.title.clone()}</a> }).collect_view()}</td>
                            </tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
            </details>
        </section>
    }
}
