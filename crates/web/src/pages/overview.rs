//! Landing page: what the knowledge base is, how statuses read, and the entry points.
use crate::components::badges::StatusBadge;
use crate::components::cards::EntryCard;
use crate::components::motif::CipherField;
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

    view! {
        <section class="hero">
            <div class="wrap hero-grid">
                <div class="hero-copy">
                    <p class="eyebrow mono">"Dilate · Cryptography Knowledge Base"</p>
                    <h1 class="display display-xl">"Cryptography, made "<em class="serif">"operational"</em>"."</h1>
                    <p class="lede">
                        "Cipher Praxis preserves the models, algebra, attack strategies, statistical instruments, solvers and validation protocols developed in a long cryptanalytic research program, and turns them into pages you can read, equations you can check, and code you can run in your browser."
                    </p>
                    <div class="hero-actions">
                        <a class="btn btn-primary" href="/ciphers">"Start with the cipher taxonomy"</a>
                        <a class="btn btn-ghost" href="/labs">"Open a WebAssembly lab"</a>
                        <button type="button" class="btn btn-ghost" on:click=move |_| palette.set(true)>"Search "<kbd class="kbd">"⌘K"</kbd></button>
                    </div>
                    <dl class="hero-stats mono">
                        <div><dt>"entries"</dt><dd>{total}</dd></div>
                        <div><dt>"sections"</dt><dd>{Section::ALL.len()}</dd></div>
                        <div><dt>"evidence files cited"</dt><dd>{prov_files}</dd></div>
                        <div><dt>"runtime"</dt><dd>"Rust → WASM"</dd></div>
                    </dl>
                </div>
                <div class="hero-visual glass">
                    <CipherField rows=9 cols=13/>
                    <div class="hero-caption mono">"tabula recta · row r, column c ↦ (r + c) mod 26"</div>
                </div>
            </div>
        </section>

        <section class="wrap section-block" aria-labelledby="ia-h">
            <div class="section-head">
                <p class="eyebrow mono">"Information architecture"</p>
                <h2 id="ia-h" class="display">"Eleven sections, one evidence trail."</h2>
                <p class="lede-sm">"Every page states what a method is, how it is attacked, where it fails, which controls were run and which files back the numbers."</p>
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
                <h2 id="status-h" class="display">"Status is a claim about evidence, not a verdict about the cipher."</h2>
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
                <div class="rule"><span class="rule-n mono">"01"</span><p><b>"Controls first."</b>" A solver prints its planted controls before any target row; a control that does not read as language means the instrument is broken."</p></div>
                <div class="rule"><span class="rule-n mono">"02"</span><p><b>"Numbers trace to receipts."</b>" Every pass rate, z-score and throughput figure names the log, audit or note it came from."</p></div>
                <div class="rule"><span class="rule-n mono">"03"</span><p><b>"Negatives are scoped."</b>" A closure says what was excluded, at what text length, with what control power. It never claims a universal impossibility."</p></div>
                <div class="rule"><span class="rule-n mono">"04"</span><p><b>"Gates must be able to fail."</b>" A check is cited only after it has been shown to reject something; nulls that satisfy the model are run alongside nulls that violate it."</p></div>
            </div>
        </section>

        <section class="wrap section-block" aria-labelledby="featured-h">
            <div class="section-head">
                <p class="eyebrow mono">"Entry points"</p>
                <h2 id="featured-h" class="display">"Start anywhere."</h2>
            </div>
            <div class="card-grid">
                {featured.into_iter().map(|e| view! { <EntryCard entry=e/> }).collect_view()}
            </div>
        </section>

        {(!labs.is_empty()).then(|| view! {
            <section class="wrap section-block" aria-labelledby="labs-h">
                <div class="section-head">
                    <p class="eyebrow mono">"Interactive"</p>
                    <h2 id="labs-h" class="display">"Labs compiled to WebAssembly."</h2>
                    <p class="lede-sm">"The same Rust code that is unit-tested natively runs here, in your browser, with no server round-trips."</p>
                </div>
                <div class="card-grid">
                    {labs.into_iter().map(|e| view! { <EntryCard entry=e/> }).collect_view()}
                </div>
            </section>
        })}
    }
}
