//! Entry pages: method pages (blocks + evidence), lab pages (instrument + notes), glossary terms.
use crate::components::badges::StatusBadge;
use crate::components::cards::RowList;
use crate::components::evidence::EvidencePanel;
use crate::components::markdown::Markdown;
use crate::labs::render_lab;
use crate::pages::not_found::NotFound;
use crate::state::use_state;
use crate::util::set_title;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use praxis_core::content::{BlockKind, Entry, Section};

#[component]
pub fn EntryPage() -> impl IntoView {
    let params = use_params_map();
    let key = Memo::new(move |_| {
        params.with(|p| {
            (
                p.get("section").unwrap_or_default(),
                p.get("id").unwrap_or_default(),
            )
        })
    });
    let state = use_state();
    let catalog = state.catalog.clone();
    view! {
        {move || {
            let (section, id) = key.get();
            match (Section::from_slug(&section), catalog.get(&id).cloned()) {
                (Some(s), Some(e)) if e.section == s => {
                    set_title(&e.title);
                    view! { <EntryView entry=e/> }.into_any()
                }
                _ => view! { <NotFound/> }.into_any(),
            }
        }}
    }
}

#[component]
fn EntryView(entry: Entry) -> impl IntoView {
    let state = use_state();
    let catalog = state.catalog.clone();
    let related: Vec<Entry> = catalog.related(&entry).into_iter().cloned().collect();
    let backlinks: Vec<Entry> = catalog.backlinks(&entry.id).into_iter().cloned().collect();
    let has_backlinks = !backlinks.is_empty();
    let backlinks_len = backlinks.len();
    let section = entry.section;
    let is_lab = section == Section::Labs;
    let is_glossary = section == Section::Glossary;
    let mut anchor_counts = std::collections::HashMap::new();
    let toc: Vec<(String, String)> = entry
        .blocks
        .iter()
        .map(|b| {
            let count = anchor_counts.entry(b.kind.slug()).or_insert(0usize);
            *count += 1;
            // Preserve existing links to the first block of each kind.
            let base = format!("{}-{}", b.kind.slug(), b.kind.ordinal());
            let id = if *count == 1 {
                base
            } else {
                format!("{base}-{count}")
            };
            (id, b.heading())
        })
        .collect();
    let block_anchors: Vec<_> = toc.iter().map(|(id, _)| id.clone()).collect();
    let lab_key = entry.lab.clone();
    let blocks = entry.blocks.clone();
    let provenance = entry.provenance.clone();
    let references = entry.references.clone();
    let tags = entry.tags.clone();
    let title = entry.title.clone();
    let subtitle = entry.subtitle.clone();
    let summary = entry.summary.clone();
    let status = entry.status;
    let status_note = entry.status_note.clone();
    let family = entry.family_or_default().to_string();
    let updated = entry.updated.clone();

    view! {
        <article class="wrap entry" class:entry-lab=is_lab>
            <div class="entry-grid">
                <div class="entry-main">
                    <header class="entry-header">
                        <div class="entry-topline">
                            <nav class="breadcrumbs" aria-label="Breadcrumb">
                                <ol>
                                    <li><a href="/">"Overview"</a></li>
                                    <li><a href=format!("/{}", section.slug())>{section.title()}</a></li>
                                    <li class="sr-only" aria-current="page">{title.clone()}</li>
                                </ol>
                            </nav>
                            {updated.map(|u| view! {
                                <p class="entry-updated">"Updated "<time datetime=u.clone()>{u.clone()}</time></p>
                            })}
                        </div>
                        <p class="entry-family">{family}</p>
                        <h1 class="display">{title.clone()}</h1>
                        {subtitle.map(|s| view! { <p class="entry-subtitle serif">{s}</p> })}
                        <div class="entry-status glass">
                            <StatusBadge status=status large=true/>
                            <div class="entry-status-note" inner_html=praxis_core::render::markdown_to_html(&status_note.unwrap_or_else(|| status.description().to_string()))></div>
                        </div>
                        {(section.is_method_section() && !is_lab).then(|| view! {
                            <details class="entry-context">
                                <summary>"How to interpret the evidence"</summary>
                                <p><b>{status.label()}</b>" means: "{status.description()}" "<a href="/#status-h">"See all statuses."</a></p>
                                <p>
                                    "A derivation establishes a claim under its stated assumptions. "
                                    <a href="/validation/planted-controls">"Planted controls"</a>
                                    " test recovery on generated cases with known answers; a "
                                    <a href="/statistics/shuffled-null-z-score">"z-score"</a>
                                    " compares a statistic with a specified null distribution. Experimental results apply to the reported model, data and budget. Exactness, recovery power and historical novelty are separate claims; a status badge does not establish all three."
                                </p>
                            </details>
                        })}
                        <div class="lede" inner_html=praxis_core::render::markdown_to_html(&summary)></div>
                        {(!tags.is_empty()).then(|| view! { <div class="card-tags">{tags.iter().map(|t| view! { <span class="tag">{t.clone()}</span> }).collect_view()}</div> })}
                    </header>
                    {(!toc.is_empty()).then(|| view! {
                        <nav class="toc toc-strip" aria-label="On this page">
                            {toc.iter().map(|(id, h)| view! { <a href=format!("#{id}")>{h.clone()}</a> }).collect_view()}
                        </nav>
                    })}
                    {lab_key.map(|k| view! {
                        <section class="lab-panel" aria-label="Interactive lab">
                            <div class="lab-panel-head mono"><span class="lab-live"><i></i>"Interactive lab"</span><span>{format!("lab/{k}")}</span></div>
                            {render_lab(&k)}
                        </section>
                    })}
                    {blocks.into_iter().zip(block_anchors).enumerate().map(|(index, (b, id))| {
                        view! {
                            <section class="block" id=id.clone() aria-labelledby=format!("{id}-h")>
                                <h2 class="block-h" id=format!("{id}-h")><span class="block-n mono">{format!("{:02}", index + 1)}</span>{b.heading()}</h2>
                                {if b.kind == BlockKind::Pseudocode {
                                    view! { <div class="prose" inner_html=praxis_core::render::pseudocode_to_html(&b.body)></div> }.into_any()
                                } else {
                                    view! { <Markdown source=b.body.clone()/> }.into_any()
                                }}
                            </section>
                        }
                    }).collect_view()}
                    {is_glossary.then(|| view! { <p class="prose">"See the related entries for examples of this term in use."</p> })}
                </div>
                <aside class="entry-side">
                    {(!toc.is_empty()).then(|| view! {
                        <nav class="toc" aria-label="On this page">
                            <h2 class="side-h">"On this page"</h2>
                            <ol>
                                {toc.iter().map(|(id, h)| view! { <li><a href=format!("#{id}")>{h.clone()}</a></li> }).collect_view()}
                                <li class="toc-end"><a href="#evidence-h">"Evidence"</a></li>
                                {has_backlinks.then(|| view! { <li class="toc-end"><a href="#cited-h">"Cited by"</a></li> })}
                            </ol>
                        </nav>
                    })}
                    {(!related.is_empty()).then(|| view! {
                        <section class="related" aria-labelledby="related-h">
                            <h2 id="related-h" class="side-h">"Related"</h2>
                            <RowList entries=related limit=4/>
                        </section>
                    })}
                </aside>
            </div>

            <div class="entry-endmatter">
                <EvidencePanel provenance=provenance references=references wide=true entry_id=entry.id.clone() entry_title=title.clone()/>
                {has_backlinks.then(|| view! {
                    <section class="related row-grid" aria-labelledby="cited-h">
                        <h2 id="cited-h" class="side-h">"Cited by"<span class="mono meta">{format!(" · {}", backlinks_len)}</span></h2>
                        <RowList entries=backlinks limit=12/>
                    </section>
                })}
            </div>
        </article>
    }
}
