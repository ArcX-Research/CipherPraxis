//! Section index pages: method listings with filters, the labs index, glossary and references.
use crate::components::badges::StatusBadge;
use crate::components::cards::EntryCard;
use crate::pages::not_found::NotFound;
use crate::state::{use_state, CORPUS_ROOT};
use crate::util::set_title;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use praxis_core::content::{Entry, Section, Status};
use praxis_core::search::tokenize;
use std::collections::BTreeMap;

#[component]
pub fn SectionPage() -> impl IntoView {
    let params = use_params_map();
    let slug = Memo::new(move |_| params.with(|p| p.get("section").unwrap_or_default()));
    view! {
        {move || match Section::from_slug(&slug.get()) {
            Some(Section::Glossary) => view! { <GlossaryPage/> }.into_any(),
            Some(Section::References) => view! { <ReferencesPage/> }.into_any(),
            Some(section) => view! { <MethodIndex section=section/> }.into_any(),
            None => view! { <NotFound/> }.into_any(),
        }}
    }
}

#[component]
fn SectionHeader(section: Section, count: usize) -> impl IntoView {
    view! {
        <header class="section-header">
            <p class="eyebrow mono">{format!("{:02} / {}", section.ordinal(), Section::ALL.len())}<span class="sep">"·"</span>{format!("{count} entries")}</p>
            <h1 class="display">{section.title()}</h1>
            <p class="lede">{section.blurb()}</p>
        </header>
    }
}

#[component]
fn MethodIndex(section: Section) -> impl IntoView {
    set_title(section.title());
    let state = use_state();
    let catalog = state.catalog.clone();
    let entries: Vec<Entry> = catalog.section(section).into_iter().cloned().collect();
    let families = catalog.families(section);
    let tags = catalog.tags(section);
    let count = entries.len();

    let status_filter = RwSignal::new(Option::<Status>::None);
    let family_filter = RwSignal::new(String::new());
    let tag_filter = RwSignal::new(String::new());
    let text_filter = RwSignal::new(String::new());
    let group_by_family = RwSignal::new(true);

    let entries_for_filter = entries.clone();
    let filtered = Memo::new(move |_| {
        let sf = status_filter.get();
        let ff = family_filter.get();
        let tf = tag_filter.get();
        let terms = tokenize(&text_filter.get());
        entries_for_filter
            .iter()
            .filter(|e| sf.map(|s| e.status == s).unwrap_or(true))
            .filter(|e| ff.is_empty() || e.family_or_default() == ff)
            .filter(|e| tf.is_empty() || e.tags.iter().any(|t| *t == tf))
            .filter(|e| {
                if terms.is_empty() {
                    return true;
                }
                let hay = tokenize(&format!(
                    "{} {} {} {}",
                    e.title,
                    e.subtitle.clone().unwrap_or_default(),
                    e.summary,
                    e.tags.join(" ")
                ));
                terms.iter().all(|t| hay.iter().any(|h| h.starts_with(t)))
            })
            .cloned()
            .collect::<Vec<Entry>>()
    });

    let status_counts: Vec<(Status, usize)> = Status::ALL
        .iter()
        .map(|s| (*s, entries.iter().filter(|e| e.status == *s).count()))
        .filter(|(_, n)| *n > 0)
        .collect();

    view! {
        <section class="wrap page">
            <SectionHeader section=section count=count/>
            <div class="filters glass" role="region" aria-label="Filters">
                <div class="filter-row">
                    <label class="filter-search">
                        <span class="sr-only">"Filter by text"</span>
                        <span class="filter-icon" aria-hidden="true" inner_html=crate::components::motif::ICON_SEARCH></span>
                        <input type="search" placeholder="Filter this section…" prop:value=move || text_filter.get() on:input=move |ev| text_filter.set(event_target_value(&ev))/>
                    </label>
                    <label class="filter-select">
                        <span class="mono meta">"family"</span>
                        <select on:change=move |ev| family_filter.set(event_target_value(&ev))>
                            <option value="">"All families"</option>
                            {families.iter().map(|f| view! { <option value=f.clone()>{f.clone()}</option> }).collect_view()}
                        </select>
                    </label>
                    <label class="filter-toggle">
                        <input type="checkbox" prop:checked=move || group_by_family.get() on:change=move |ev| group_by_family.set(event_target_checked(&ev))/>
                        <span>"Group by family"</span>
                    </label>
                </div>
                <div class="filter-row chips" role="group" aria-label="Filter by status">
                    <button type="button" class="chip" class:active=move || status_filter.get().is_none() on:click=move |_| status_filter.set(None)>"All"<span class="chip-n mono">{count}</span></button>
                    {status_counts.into_iter().map(|(s, n)| view! {
                        <button type="button" class=format!("chip chip-{}", s.slug()) class:active=move || status_filter.get() == Some(s) on:click=move |_| status_filter.set(if status_filter.get() == Some(s) { None } else { Some(s) }) aria-pressed=move || (status_filter.get() == Some(s)).to_string()>
                            {s.label()}<span class="chip-n mono">{n}</span>
                        </button>
                    }).collect_view()}
                </div>
                {(!tags.is_empty()).then(|| view! {
                    <div class="filter-row chips chips-tags" role="group" aria-label="Filter by tag">
                        {tags.iter().take(18).map(|(t, n)| {
                            let t1 = t.clone();
                            let t2 = t.clone();
                            let t3 = t.clone();
                            view! {
                                <button type="button" class="chip chip-tag" class:active=move || tag_filter.get() == t1 on:click=move |_| tag_filter.set(if tag_filter.get() == t2 { String::new() } else { t3.clone() })>
                                    {t.clone()}<span class="chip-n mono">{*n}</span>
                                </button>
                            }
                        }).collect_view()}
                    </div>
                })}
            </div>
            <p class="mono meta result-count">{move || format!("{} of {} shown", filtered.with(|f| f.len()), count)}</p>
            {move || {
                let list = filtered.get();
                if list.is_empty() {
                    return view! { <p class="empty">"No entries match these filters."</p> }.into_any();
                }
                if group_by_family.get() {
                    let mut groups: BTreeMap<String, Vec<Entry>> = BTreeMap::new();
                    for e in list {
                        groups.entry(e.family_or_default().to_string()).or_default().push(e);
                    }
                    groups.into_iter().map(|(family, items)| view! {
                        <div class="family-group">
                            <h2 class="family-h"><span class="mono meta">{format!("{:02}", items.len())}</span>{family}</h2>
                            <div class="card-grid">{items.into_iter().map(|e| view! { <EntryCard entry=e/> }).collect_view()}</div>
                        </div>
                    }).collect_view().into_any()
                } else {
                    view! { <div class="card-grid">{list.into_iter().map(|e| view! { <EntryCard entry=e/> }).collect_view()}</div> }.into_any()
                }
            }}
        </section>
    }
}

#[component]
fn GlossaryPage() -> impl IntoView {
    set_title("Glossary");
    let state = use_state();
    let catalog = state.catalog.clone();
    let mut terms: Vec<Entry> = catalog
        .section(Section::Glossary)
        .into_iter()
        .cloned()
        .collect();
    terms.sort_by_key(|e| e.title.to_lowercase());
    let count = terms.len();
    let filter = RwSignal::new(String::new());
    let letters: Vec<char> = {
        let mut v: Vec<char> = terms
            .iter()
            .filter_map(|t| t.title.chars().next())
            .map(|c| c.to_ascii_uppercase())
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    };
    let terms_all = terms.clone();
    let shown = Memo::new(move |_| {
        let q = tokenize(&filter.get());
        terms_all
            .iter()
            .filter(|t| {
                q.is_empty() || {
                    let hay = tokenize(&format!("{} {}", t.title, t.summary));
                    q.iter().all(|w| hay.iter().any(|h| h.starts_with(w)))
                }
            })
            .cloned()
            .collect::<Vec<_>>()
    });
    let catalog2 = catalog.clone();
    view! {
        <section class="wrap page">
            <SectionHeader section=Section::Glossary count=count/>
            <div class="glossary-tools glass">
                <label class="filter-search">
                    <span class="sr-only">"Filter terms"</span>
                    <span class="filter-icon" aria-hidden="true" inner_html=crate::components::motif::ICON_SEARCH></span>
                    <input type="search" placeholder="Filter terms…" prop:value=move || filter.get() on:input=move |ev| filter.set(event_target_value(&ev))/>
                </label>
                <nav class="letter-nav mono" aria-label="Jump to letter">
                    {letters.iter().map(|c| view! { <a href=format!("#g-{c}")>{c.to_string()}</a> }).collect_view()}
                </nav>
            </div>
            <dl class="glossary">
                {move || {
                    let mut last = ' ';
                    shown.get().into_iter().map(|t| {
                        let first = t.title.chars().next().map(|c| c.to_ascii_uppercase()).unwrap_or('#');
                        let anchor = if first != last { last = first; Some(first) } else { None };
                        let related: Vec<Entry> = catalog2.related(&t).into_iter().cloned().collect();
                        let id = t.id.clone();
                        view! {
                            {anchor.map(|c| view! { <div class="glossary-letter mono" id=format!("g-{c}")>{c.to_string()}</div> })}
                            <div class="glossary-item" id=id>
                                <dt><a href=t.route()>{t.title.clone()}</a>{t.family.clone().map(|f| view! { <span class="mono meta">{f}</span> })}</dt>
                                <dd>
                                    <p>{t.summary.clone()}</p>
                                    {(!related.is_empty()).then(|| view! {
                                        <p class="glossary-related">"See "{related.iter().enumerate().map(|(i, r)| view! { {(i > 0).then(|| ", ")}<a href=r.route()>{r.title.clone()}</a> }).collect_view()}"."</p>
                                    })}
                                </dd>
                            </div>
                        }
                    }).collect_view()
                }}
            </dl>
        </section>
    }
}

#[component]
fn ReferencesPage() -> impl IntoView {
    set_title("References");
    let state = use_state();
    let catalog = state.catalog.clone();
    let refs: Vec<Entry> = catalog
        .section(Section::References)
        .into_iter()
        .cloned()
        .collect();
    let count = refs.len();
    let prov = catalog.provenance_map();
    let prov_rows: Vec<(String, Vec<Entry>)> = prov
        .into_iter()
        .map(|(p, v)| (p, v.into_iter().cloned().collect()))
        .collect();
    let prov_count = prov_rows.len();
    view! {
        <section class="wrap page">
            <SectionHeader section=Section::References count=count/>
            <div class="ref-groups">
                {refs.into_iter().map(|e| {
                    let list = e.references.clone();
                    view! {
                        <article class="ref-group glass" id=e.id.clone()>
                            <div class="card-top"><span class="mono meta">{e.family_or_default().to_string()}</span><StatusBadge status=e.status/></div>
                            <h2 class="ref-group-title"><a href=e.route()>{e.title.clone()}</a></h2>
                            <p class="card-summary">{e.summary.clone()}</p>
                            {(!list.is_empty()).then(|| view! {
                                <ol class="ref-list">
                                    {list.into_iter().map(|r| {
                                        let meta = [r.author.clone(), r.year.map(|y| y.to_string())].into_iter().flatten().collect::<Vec<_>>().join(", ");
                                        view! {
                                            <li>
                                                {match r.url.clone() { Some(u) => view! { <a href=u target="_blank" rel="noopener">{r.title.clone()}</a> }.into_any(), None => view! { <span class="ref-title">{r.title.clone()}</span> }.into_any() }}
                                                {(!meta.is_empty()).then(|| view! { <span class="ref-meta">{format!(" — {meta}")}</span> })}
                                                {r.note.clone().map(|n| view! { <div class="prov-note">{n}</div> })}
                                            </li>
                                        }
                                    }).collect_view()}
                                </ol>
                            })}
                        </article>
                    }
                }).collect_view()}
            </div>
            <div class="section-head">
                <p class="eyebrow mono">"Evidence corpus index"</p>
                <h2 class="display-sm">{format!("{prov_count} corpus files are cited across the knowledge base.")}</h2>
                <p class="lede-sm">"Paths are relative to the corpus root "<code>{CORPUS_ROOT}</code>". The corpus is internal; these pointers exist so that every number on a page can be traced."</p>
            </div>
            <div class="table-scroll">
                <table class="prov-table">
                    <thead><tr><th>"Corpus file"</th><th>"Cited by"</th></tr></thead>
                    <tbody>
                        {prov_rows.into_iter().map(|(p, es)| view! {
                            <tr>
                                <td><code>{p}</code></td>
                                <td>{es.iter().enumerate().map(|(i, e)| view! { {(i > 0).then(|| ", ")}<a href=e.route()>{e.title.clone()}</a> }).collect_view()}</td>
                            </tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
        </section>
    }
}
