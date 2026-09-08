use super::widgets::{histogram_svg, ErrorNote, Note, Output, TextField};
use leptos::prelude::*;
use praxis_core::crypto::periodic::{cosets, decrypt, encrypt, Variant};
use praxis_core::crypto::stats::{counts, ic_of_counts, recover_shifts};

#[component]
pub fn VigenereLab() -> impl IntoView {
    let plain = RwSignal::new("The quick brown fox jumps over the lazy dog while the sleeping guard dreams of forgotten keys and repeating alphabets".to_string());
    let key = RwSignal::new("LEMON".to_string());
    let variant = RwSignal::new(Variant::Vigenere);
    let cipher = Memo::new(move |_| encrypt(&plain.get(), &key.get(), variant.get()));
    let cipher_text = Signal::derive(move || cipher.get().unwrap_or_default());
    let error = Signal::derive(move || cipher.get().err());
    let back = Signal::derive(move || {
        cipher
            .get()
            .ok()
            .and_then(|c| decrypt(&c, &key.get(), variant.get()).ok())
            .unwrap_or_default()
    });
    let m = Memo::new(move |_| {
        key.get()
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .count()
            .max(1)
    });
    let recovered = Memo::new(move |_| {
        let c = cipher_text.get();
        if variant.get() != Variant::Vigenere || c.len() < 5 * m.get() {
            return None;
        }
        Some(
            recover_shifts(&c, m.get())
                .iter()
                .map(|&s| (b'A' + s as u8) as char)
                .collect::<String>(),
        )
    });
    view! {
        <div class="lab">
            <div class="lab-controls">
                <TextField label="Plaintext" value=plain multiline=true/>
                <div class="lab-fields">
                    <TextField label="Key" value=key mono=true hint="Use letters only. The key repeats."/>
                    <div class="field">
                        <label class="field-label" for="variant">"Rule"</label>
                        <select id="variant" class="field-input" on:change=move |ev| variant.set(Variant::from_slug(&event_target_value(&ev)).unwrap_or(Variant::Vigenere))>
                            {Variant::ALL.into_iter().map(|v| view! { <option value=v.slug() selected=move || variant.get() == v>{format!("{} — {}", v.label(), v.formula())}</option> }).collect_view()}
                        </select>
                    </div>
                </div>
            </div>
            <ErrorNote message=error/>
            <Output label="Ciphertext" value=cipher_text/>
            <Output label="Decrypted with the same key" value=back/>
            <div class="lab-viz">
                <div class="lab-viz-head mono">{move || format!("Letter counts for each coset · period m = {} · IC per coset", m.get())}</div>
                <div class="coset-grid">
                    {move || {
                        let c = cipher_text.get();
                        cosets(&c, m.get()).into_iter().enumerate().map(|(i, cs)| {
                            let ct = counts(&cs);
                            let ic = ic_of_counts(&ct);
                            view! {
                                <div class="coset">
                                    <div class="mono meta">{format!("coset {i} · n = {} · IC {ic:.3}", cs.len())}</div>
                                    <div inner_html=histogram_svg(&ct, &format!("coset {i}"))></div>
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>
            {move || recovered.get().map(|r| view! {
                <Note>"A blind χ² test compares each coset with English letter frequencies and returns the key "<code class="mono">{r}</code>". It works when every coset contains enough text for its shifted frequency pattern to be clear."</Note>
            })}
            <Note>"Each coset is a Caesar shift of the plaintext letters in those positions. Its tallest bar is often E shifted by the matching key letter. Shorten the text or lengthen the key to see the frequency patterns flatten and key recovery fail."</Note>
        </div>
    }
}
