use super::widgets::{ErrorNote, Note, NumberField, Output, TextField};
use leptos::prelude::*;
use praxis_core::crypto::affine::{decrypt, encrypt};
use praxis_core::crypto::alphabet::normalize;
use praxis_core::crypto::numtheory::{mod_inverse, units};

#[component]
pub fn AffineLab() -> impl IntoView {
    let plain = RwSignal::new("Affine cipher".to_string());
    let a = RwSignal::new(5i64);
    let b = RwSignal::new(8i64);
    let res = Memo::new(move |_| encrypt(&plain.get(), a.get(), b.get()));
    let cipher = Signal::derive(move || res.get().unwrap_or_default());
    let err = Signal::derive(move || res.get().err());
    let back = Signal::derive(move || {
        res.get()
            .ok()
            .and_then(|c| decrypt(&c, a.get(), b.get()).ok())
            .unwrap_or_default()
    });
    let inv = Memo::new(move |_| mod_inverse(a.get(), 26));
    let first_letter = Memo::new(move |_| {
        let text = normalize(&plain.get());
        let letter = text.bytes().next()?;
        let encrypted = res.get().ok()?.bytes().next()?;
        let p = (letter - b'A') as i64;
        let c = (encrypted - b'A') as i64;
        Some(format!(
            "{} = {p} → ({} × {p} + {}) mod 26 = {c} → {}",
            letter as char,
            a.get(),
            b.get(),
            encrypted as char
        ))
    });
    view! {
        <div class="lab affine-lab">
            <div class="lab-intro">
                <p>"Change the message and key to explore encryption and its inverse."</p>
                <button type="button" class="lab-reset" on:click=move |_| {
                    plain.set("Affine cipher".into());
                    a.set(5);
                    b.set(8);
                }>"Reset example"</button>
            </div>
            <div class="lab-fields affine-setup">
                <TextField label="Plaintext" value=plain hint="Uses A–Z; spaces and punctuation are omitted."/>
                <NumberField label="Multiplier · a" value=a min=0 max=25/>
                <NumberField label="Shift · b" value=b min=0 max=25/>
            </div>
            <ErrorNote message=err/>
            <div class="affine-results">
                <Output label="Ciphertext" formula="c = a · p + b (mod 26)" value=cipher/>
                <Output label="Recovered plaintext" formula="p = a⁻¹ · (c − b) (mod 26)" value=back grouped=false/>
            </div>
            <p class="lab-caption">"Letters use A = 0, …, Z = 25. Ciphertext is grouped in fives for readability."</p>
            {move || first_letter.get().map(|example| view! {
                <div class="affine-example">
                    <span>"First letter"</span><span class="mono">{example}</span>
                </div>
            })}
            <section class="affine-multipliers" aria-labelledby="affine-multipliers-h">
                <div class="affine-multipliers-head">
                    <h3 id="affine-multipliers-h">"Valid multipliers"</h3>
                    <span class="mono">"gcd(a, 26) = 1"</span>
                </div>
                <p class="lab-caption">"Choose a value below. Its modular inverse is shown underneath."</p>
                <div class="unit-row">
                    {units(26).into_iter().map(|u| {
                        let inverse = mod_inverse(u, 26).unwrap();
                        view! {
                            <button type="button" class="unit" class:active=move || a.get() == u
                                aria-pressed=move || (a.get() == u).to_string()
                                aria-label=format!("Use multiplier {u}, inverse {inverse}")
                                on:click=move |_| a.set(u)>
                                {u}<span class="unit-inv">{format!("a⁻¹={inverse}")}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>
                <div class="affine-inverse mono">{move || match inv.get() {
                    Some(i) => format!("a⁻¹ = {i}; {} × {i} = {} ≡ 1 (mod 26)", a.get(), a.get() * i),
                    None => "Choose a valid multiplier to obtain an inverse.".into(),
                }}</div>
            </section>
            <Note><strong>"312 possible keys."</strong>" Twelve valid multipliers × 26 shifts. Every key can be tried in an exhaustive search."</Note>
        </div>
    }
}
