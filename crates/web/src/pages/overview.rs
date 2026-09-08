//! Landing page: what the knowledge base is, how statuses read, and the entry points.
use crate::components::badges::StatusBadge;
use crate::components::cards::EntryCard;
use crate::components::motif::CipherDisc;
use crate::state::{use_palette, use_state};
use crate::util::set_title;
use leptos::prelude::*;
use praxis_core::content::{Section, Status};

#[component]
pub fn Overview() -> impl IntoView {
    set_title("Overview");
    let state = use_state();
    let palette = use_palette();
    let catalog = state.catalog.clone();
    let total = catalog.len();
    let counts = catalog.status_counts();
    let labs: Vec<_> = catalog
        .section(Section::Labs)
        .into_iter()
        .take(3)
        .cloned()
        .collect();
    let featured: Vec<_> = [
        "vigenere",
        "index-of-coincidence",
        "planted-controls",
        "simulated-annealing",
        "quagmire-iii",
        "modular-inverse-and-crt",
    ]
    .iter()
    .filter_map(|id| catalog.get(id).cloned())
    .collect();
    let prov_files = catalog.provenance_map().len();
    let lab_count = catalog.section_count(Section::Labs);

    view! {
        <section class="hero">
            <div class="wrap hero-grid">
                <div class="hero-copy">
                    <p class="eyebrow mono">"Dilate · Cryptography knowledge base"</p>
                    <h1 class="display display-xl">"Cryptography,"<br/><em class="serif">"from first principles"</em></h1>
                    <p class="lede">
                        "Cipher systems, mathematical foundations and methods of cryptanalysis. Explore precise definitions, optimized pseudocode, worked examples and the evidence behind each claim."
                    </p>
                    <div class="hero-actions">
                        <a class="btn btn-primary" href="/ciphers">"Browse ciphers"</a>
                        <a class="btn btn-ghost" href="/labs">"Try a lab"</a>
                        <button type="button" class="btn btn-ghost" on:click=move |_| palette.set(true)>"Search "<kbd class="kbd">"⌘K"</kbd></button>
                    </div>
                    <dl class="hero-stats mono">
                        <div><dt>"entries"</dt><dd>{total}</dd></div>
                        <div><dt>"sections"</dt><dd>{Section::ALL.len()}</dd></div>
                        <div><dt>"evidence files"</dt><dd>{prov_files}</dd></div>
                        <div><dt>"interactive labs"</dt><dd>{lab_count}</dd></div>
                    </dl>
                </div>
                <div class="hero-visual"><CipherDisc/></div>
            </div>
        </section>

        <section class="wrap section-block" aria-labelledby="ia-h">
            <div class="section-head">
                <p class="eyebrow mono">"Browse the knowledge base"</p>
                <h2 id="ia-h" class="display">"Explore ciphers, methods, and tools"</h2>
                <p class="lede-sm">"Each page explains the method, how to test or attack it, when it can fail, which checks were run, and where the numbers came from."</p>
            </div>
            <div class="ia-grid">
                {Section::ALL.into_iter().map(|s| {
                    let n = catalog.section_count(s);
                    view! {
                        <a class="ia-card" href=format!("/{}", s.slug())>
                            <span class="ia-ord mono">{format!("{:02}", s.ordinal())}</span>
                            <span class="ia-title">{s.title()}</span>
                            <span class="ia-blurb">{s.blurb()}</span>
                            <span class="ia-count mono">{format!("{n} entries")}</span>
                        </a>
                    }
                }).collect_view()}
            </div>
        </section>

        <section class="wrap section-block" aria-labelledby="status-h">
            <div class="section-head">
                <p class="eyebrow mono">"Reading a page"</p>
                <h2 id="status-h" class="display">"A status tells you what the evidence supports"</h2>
            </div>
            <div class="status-grid">
                {Status::ALL.into_iter().map(|s| {
                    let n = counts.iter().find(|(st, _)| *st == s).map(|(_, n)| *n).unwrap_or(0);
                    view! {
                        <div class="status-card">
                            <div class="status-card-top"><StatusBadge status=s large=true/><span class="mono meta">{format!("{n}")}</span></div>
                            <p>{s.description()}</p>
                        </div>
                    }
                }).collect_view()}
            </div>
            <div class="rules glass">
                <div class="rule"><span class="rule-n mono">"01"</span><p><b>"Run controls first."</b>" Define what each control should recover and test it with the same procedure. A failed control limits what a target miss can establish."</p></div>
                <div class="rule"><span class="rule-n mono">"02"</span><p><b>"Trace every number."</b>" Each pass rate, z-score, and speed figure points to its source log, audit, or note."</p></div>
                <div class="rule"><span class="rule-n mono">"03"</span><p><b>"Keep negative claims narrow."</b>" State the tested family, assumptions, text length, and recovery rate. Distinguish an exhaustive exclusion from a search that found no answer."</p></div>
                <div class="rule"><span class="rule-n mono">"04"</span><p><b>"Test failure modes."</b>" Use matching positive and negative controls. Separate null cases that satisfy the model from cases that violate its assumptions."</p></div>
            </div>
        </section>

        <section class="wrap section-block" aria-labelledby="featured-h">
            <div class="section-head">
                <p class="eyebrow mono">"Entry points"</p>
                <h2 id="featured-h" class="display">"Start anywhere"</h2>
            </div>
            <div class="card-grid">
                {featured.into_iter().map(|e| view! { <EntryCard entry=e/> }).collect_view()}
            </div>
        </section>

        {(!labs.is_empty()).then(|| view! {
            <section class="wrap section-block" aria-labelledby="labs-h">
                <div class="section-head">
                    <p class="eyebrow mono">"Try it yourself"</p>
                    <h2 id="labs-h" class="display">"Run the labs in your browser"</h2>
                    <p class="lede-sm">"The labs use the same tested Rust code as the main project. They run on your device and do not send work to a server."</p>
                </div>
                <div class="card-grid">
                    {labs.into_iter().map(|e| view! { <EntryCard entry=e/> }).collect_view()}
                </div>
            </section>
        })}
    }
}
