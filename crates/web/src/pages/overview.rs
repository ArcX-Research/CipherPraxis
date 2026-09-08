//! Overview: a welcoming introduction, concise subject cards, labs and an evidence guide.
use crate::components::badges::StatusBadge;
use crate::state::use_state;
use crate::util::set_title;
use leptos::prelude::*;
use praxis_core::content::{Section, Status};

#[component]
pub fn Overview() -> impl IntoView {
    set_title("Overview");
    let catalog = use_state().catalog.clone();
    let total = catalog.len();
    let lab_count = catalog.section_count(Section::Labs);
    // These are introductions to existing entries, not separate copies of their content.
    let starting_points: Vec<_> = [
        (
            "vigenere",
            "Understand",
            "How does a cipher work?",
            "Follow a repeating key from plaintext to ciphertext, then explore what it reveals.",
        ),
        (
            "index-of-coincidence",
            "Investigate",
            "What can a pattern reveal?",
            "See how matching letters help distinguish random text from structured signals.",
        ),
        (
            "planted-controls",
            "Evaluate",
            "What makes a result convincing?",
            "Use cases with known answers to find out what an analysis can actually recover.",
        ),
    ]
    .into_iter()
    .filter_map(|(id, label, question, description)| {
        catalog
            .get(id)
            .cloned()
            .map(|entry| (entry, label, question, description))
    })
    .collect();
    let labs: Vec<_> = [
        (
            "vigenere-lab",
            "Substitution",
            "Change the key and compare four related cipher systems.",
        ),
        (
            "coincidence-lab",
            "Pattern analysis",
            "Explore repeated letters, matching gaps and possible key periods.",
        ),
        (
            "hill-lab",
            "Linear algebra",
            "Transform letter blocks with a matrix and test whether it has an inverse.",
        ),
    ]
    .into_iter()
    .filter_map(|(id, label, description)| {
        catalog
            .get(id)
            .cloned()
            .map(|entry| (entry, label, description))
    })
    .collect();

    view! {
        <div class="overview">
            <section class="hero" aria-labelledby="welcome-h">
                <div class="wrap hero-grid">
                    <div class="hero-copy">
                        <p class="eyebrow">"A field guide to cryptography"</p>
                        <h1 id="welcome-h" class="display display-xl">"The science of"<br/><em class="serif">"keeping secrets"</em></h1>
                        <p class="lede">"Explore how ciphers protect information, how cryptanalysis tests their limits, and what the mathematical evidence supports."</p>
                        <div class="hero-actions">
                            <a class="btn btn-primary" href="#ia-h">"Explore the library"<span aria-hidden="true">"↗"</span></a>
                            <a class="btn btn-ghost" href="/labs">"Try an interactive lab"</a>
                        </div>
                        <dl class="hero-stats">
                            <div><dt>"knowledge entries"</dt><dd>{total}</dd></div>
                            <div><dt>"subject areas"</dt><dd>{Section::ALL.len()}</dd></div>
                            <div><dt>"interactive labs"</dt><dd>{lab_count}</dd></div>
                        </dl>
                    </div>
                    <div class="hero-visual">
                        <img src="/security-hero.svg" width="756" height="520" fetchpriority="high"
                            alt="A security shield connects through smaller locks to a message, illustrating protected communication."/>
                    </div>
                </div>
            </section>

            <section class="wrap overview-section overview-start" aria-labelledby="start-h">
                <div class="overview-section-head">
                    <div><p class="eyebrow">"A place to begin"</p><h2 id="start-h">"Start with a question"</h2></div>
                    <p>"Clear explanations. Worked examples. Ideas you can test."</p>
                </div>
                <div class="starting-grid">
                    {starting_points.into_iter().map(|(entry, label, question, description)| view! {
                        <a class="starting-card" href=entry.route()>
                            <span class="starting-label">{label}</span>
                            <h3>{question}</h3>
                            <p>{description}</p>
                            <span class="starting-link">{entry.title.clone()}<span aria-hidden="true">"→"</span></span>
                        </a>
                    }).collect_view()}
                </div>
            </section>

            <section class="wrap overview-section" aria-labelledby="ia-h">
                <div class="overview-section-head">
                    <div><p class="eyebrow">"The library"</p><h2 id="ia-h">"Explore by subject"</h2></div>
                    <p>"From the structure of a cipher to the evidence behind an attack."</p>
                </div>
                <div class="subject-grid">
                    {Section::ALL.into_iter().take(8).map(|section| view! {
                        <a class="subject-card" href=format!("/{}", section.slug())>
                            <div class="subject-card-top">
                                <span class="subject-icon" aria-hidden="true"><SubjectIcon section=section/></span>
                                <span class="subject-count mono">{format!("{} entries", catalog.section_count(section))}</span>
                            </div>
                            <h3>{section.title()}</h3>
                            <p>{subject_description(section)}</p>
                            <span class="subject-arrow" aria-hidden="true">"↗"</span>
                        </a>
                    }).collect_view()}
                </div>
                <nav class="resource-links" aria-label="Practice and reference">
                    {[
                        (Section::Labs, "Experiment with an idea"),
                        (Section::Glossary, "Find the right definition"),
                        (Section::References, "Follow the source material"),
                    ].into_iter().map(|(section, description)| view! {
                        <a href=format!("/{}", section.slug())>
                            <span><strong>{section.title()}</strong><span>{description}</span></span>
                            <span class="resource-arrow" aria-hidden="true">"→"</span>
                        </a>
                    }).collect_view()}
                </nav>
            </section>

            {(!labs.is_empty()).then(|| view! {
                <section class="wrap overview-section" aria-labelledby="labs-h">
                    <div class="overview-section-head">
                        <div><p class="eyebrow">"From theory to practice"</p><h2 id="labs-h">"Make the mathematics tangible"</h2></div>
                        <a class="overview-text-link" href="/labs">{format!("Explore all {lab_count} labs")}<span aria-hidden="true">"→"</span></a>
                    </div>
                    <div class="lab-teaser-grid">
                        {labs.into_iter().enumerate().map(|(index, (entry, label, description))| view! {
                            <a class="lab-teaser" href=entry.route()>
                                <div class="lab-teaser-label"><span class="mono">{format!("{:02}", index + 1)}</span>{label}</div>
                                <h3>{entry.title.clone()}</h3>
                                <p>{description}</p>
                                <span class="starting-link">"Open lab"<span aria-hidden="true">"→"</span></span>
                            </a>
                        }).collect_view()}
                    </div>
                </section>
            })}

            <section class="wrap overview-section" aria-labelledby="status-h">
                <div class="evidence-guide">
                    <div class="evidence-guide-intro">
                        <p class="eyebrow">"Read with confidence"</p>
                        <h2 id="status-h">"Every claim has a scope"</h2>
                        <p>"A status describes the support for a particular claim. Read it alongside the assumptions, controls and sources on each page."</p>
                        <a class="overview-text-link" href="/validation">"Explore validation methods"<span aria-hidden="true">"→"</span></a>
                        <details class="overview-principles" open>
                            <summary>"Four principles for reading evidence"</summary>
                            <ol>
                                <li><b>"Run controls first"</b>" Define what each control should recover and test it with the same procedure. A failed control limits what a target miss can establish."</li>
                                <li><b>"Trace every number"</b>" Follow pass rates, z-scores and speed figures to their source logs, audits or notes."</li>
                                <li><b>"Keep negative claims narrow"</b>" State the tested family, assumptions, text length and recovery rate. Distinguish an exhaustive exclusion from a search that found no answer."</li>
                                <li><b>"Test failure modes"</b>" Use matching positive and negative controls. Separate null cases that satisfy the model from cases that violate its assumptions."</li>
                            </ol>
                        </details>
                    </div>
                    <dl class="status-guide-list">
                        {Status::ALL.into_iter().map(|status| view! {
                            <div><dt><StatusBadge status=status/></dt><dd>{status.description()}</dd></div>
                        }).collect_view()}
                    </dl>
                </div>
            </section>
        </div>
    }
}

fn subject_description(section: Section) -> &'static str {
    match section {
        Section::Ciphers => {
            "Cipher families, key structures and the rules that transform a message."
        }
        Section::Algebra => "Modular arithmetic, groups and the linear algebra behind the methods.",
        Section::Cryptanalysis => "Find periods, use known text and untangle layers of encryption.",
        Section::Statistics => "Measure patterns, compare null models and quantify uncertainty.",
        Section::Search => "Explore large key spaces with heuristics and optimization.",
        Section::Exact => "Solve constraints and establish exclusions within a stated model.",
        Section::Validation => "Test recovery, audit assumptions and make results reproducible.",
        Section::Engineering => "Build efficient implementations with checks you can repeat.",
        _ => section.blurb(),
    }
}

#[component]
fn SubjectIcon(section: Section) -> impl IntoView {
    let paths = match section {
        Section::Ciphers => "M7 10V7a5 5 0 0 1 10 0v3M5 10h14v11H5ZM12 14v3",
        Section::Algebra => "M8 4H4v16h4M16 4h4v16h-4M8 9h8M12 7v4M8 16h8",
        Section::Cryptanalysis => {
            "M15 15l6 6M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0ZM7 12l2-4 2 3 2-2"
        }
        Section::Statistics => "M4 3v17h17M8 16v-4M13 16V7M18 16V4",
        Section::Search => "M4 5h6v5H4ZM14 15h6v5h-6ZM7 10v7h7M10 7h7v8",
        Section::Exact => "M4 4h16v16H4ZM4 10h16M10 4v16M13 15l2 2 3-4",
        Section::Validation => "M12 3 3 7v5c0 5 9 9 9 9s9-4 9-9V7ZM8 12l3 3 5-6",
        _ => "M8 3v3M16 3v3M8 18v3M16 18v3M3 8h3M3 16h3M18 8h3M18 16h3M6 6h12v12H6ZM10 10h4v4h-4Z",
    };
    view! { <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"><path d=paths/></svg> }
}
