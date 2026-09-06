//! Evidence panel: internal provenance (corpus files, logs, ledger ids) and literature.
use crate::components::request::EvidenceRequest;
use crate::state::CORPUS_ROOT;
use leptos::prelude::*;
use praxis_core::content::{Provenance, Reference};

#[component]
pub fn EvidencePanel(
    provenance: Vec<Provenance>,
    references: Vec<Reference>,
    #[prop(optional)] wide: bool,
    #[prop(optional, into)] entry_id: String,
    #[prop(optional, into)] entry_title: String,
) -> impl IntoView {
    let has_prov = !provenance.is_empty();
    let has_refs = !references.is_empty();
    let request_form = (has_prov && !entry_id.is_empty()).then(|| {
        let provenance = provenance.clone();
        view! { <EvidenceRequest entry_id=entry_id.clone() entry_title=entry_title.clone() provenance=provenance/> }
    });
    view! {
        <section class="evidence" class:evidence-wide=wide aria-labelledby="evidence-h">
            <h2 id="evidence-h" class="side-h"><span class="mono meta">"§"</span>" Evidence"</h2>
            {has_prov.then(|| view! {
                <p class="evidence-note">"These internal files support the claims on this page. Each path starts at "<code>{CORPUS_ROOT}</code>" unless it begins with "<code>"CipherPraxis/"</code>", which points to this website's repository."</p>
                <ul class="prov-list">
                    {provenance.into_iter().map(|p| view! {
                        <li class="prov">
                            <div class="prov-head">
                                <span class=format!("kind kind-{}", p.kind)>{p.kind.clone()}</span>
                                <code class="prov-path">{p.path.clone()}</code>
                            </div>
                            {p.reference.clone().map(|r| view! { <div class="prov-ref mono">{r}</div> })}
                            {p.note.clone().map(|n| view! { <div class="prov-note">{n}</div> })}
                        </li>
                    }).collect_view()}
                </ul>
            })}
            {request_form}
            {has_refs.then(|| view! {
                <h3 class="side-h side-h-sm">"Books and papers"</h3>
                <ul class="ref-list">
                    {references.into_iter().map(|r| {
                        let meta = [r.author.clone(), r.year.map(|y| y.to_string())]
                            .into_iter()
                            .flatten()
                            .collect::<Vec<_>>()
                            .join(", ");
                        view! {
                            <li>
                                {match r.url.clone() {
                                    Some(u) => view! { <a href=u target="_blank" rel="noopener">{r.title.clone()}</a> }.into_any(),
                                    None => view! { <span class="ref-title">{r.title.clone()}</span> }.into_any(),
                                }}
                                {(!meta.is_empty()).then(|| view! { <span class="ref-meta">{format!(" — {meta}")}</span> })}
                                {r.note.clone().map(|n| view! { <div class="prov-note">{n}</div> })}
                            </li>
                        }
                    }).collect_view()}
                </ul>
            })}
            {(!has_prov && !has_refs).then(|| view! { <p class="evidence-note">"This entry does not list any source files, books, or papers yet."</p> })}
        </section>
    }
}
