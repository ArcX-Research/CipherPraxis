use super::widgets::{Note, Output, TextField};
use leptos::prelude::*;
use praxis_core::crypto::periodic::{Quagmire, QuagmireKind};

#[component]
pub fn QuagmireLab() -> impl IntoView {
    let plain = RwSignal::new(
        "Meet at the old observatory at midnight and bring the second key".to_string(),
    );
    let plain_kw = RwSignal::new("SPRINGFEVER".to_string());
    let cipher_kw = RwSignal::new("FLOWER".to_string());
    let key = RwSignal::new("PARTY".to_string());
    let indicator = RwSignal::new("A".to_string());
    let kind = RwSignal::new(QuagmireKind::III);
    let q = Memo::new(move |_| {
        Quagmire::new(
            kind.get(),
            &plain_kw.get(),
            &cipher_kw.get(),
            &key.get(),
            indicator.get().chars().next().unwrap_or('A'),
        )
    });
    let cipher =
        Signal::derive(move || q.get().map(|q| q.encrypt(&plain.get())).unwrap_or_default());
    let back = Signal::derive(move || {
        q.get()
            .map(|q| q.decrypt(&cipher.get()))
            .unwrap_or_default()
    });
    view! {
        <div class="lab">
            <div class="lab-controls">
                <div class="lab-row">
                    <div class="field">
                        <label class="field-label" for="qkind">"Type"</label>
                        <select id="qkind" class="field-input" on:change=move |ev| kind.set(QuagmireKind::from_slug(&event_target_value(&ev)).unwrap_or(QuagmireKind::III))>
                            {QuagmireKind::ALL.into_iter().map(|k| view! { <option value=k.slug() selected=move || kind.get() == k>{format!("{} — {}", k.label(), k.description())}</option> }).collect_view()}
                        </select>
                    </div>
                    <TextField label="Key" value=key mono=true/>
                    <TextField label="Indicator" value=indicator mono=true hint="Use one letter"/>
                </div>
                <div class="lab-row">
                    <TextField label="Plaintext keyword" value=plain_kw mono=true hint="Used by types I, III, and IV"/>
                    <TextField label="Ciphertext keyword" value=cipher_kw mono=true hint="Used by types II and IV"/>
                </div>
                <TextField label="Plaintext" value=plain multiline=true/>
            </div>
            <Output label="Ciphertext" value=cipher/>
            <Output label="Decrypted" value=back/>
            <div class="lab-viz">
                <div class="lab-viz-head mono">"Tableau rows used · plaintext alphabet on top, one ciphertext row for each key letter"</div>
                {move || q.get().ok().map(|q| {
                    let pa = q.plain_alphabet.to_string();
                    let shifts = q.shifts();
                    view! {
                        <div class="tableau mono">
                            <div class="tableau-row tableau-head"><span class="tableau-key">"pt"</span><span class="tableau-cells">{pa.chars().map(|c| view! { <i>{c.to_string()}</i> }).collect_view()}</span></div>
                            {(0..shifts.len()).map(|j| {
                                let row = q.row(j);
                                let k = q.key[j] as char;
                                view! { <div class="tableau-row"><span class="tableau-key">{format!("{k} +{}", shifts[j])}</span><span class="tableau-cells">{row.chars().map(|c| view! { <i>{c.to_string()}</i> }).collect_view()}</span></div> }
                            }).collect_view()}
                        </div>
                    }
                })}
            </div>
            <Note>"When both keywords are empty, every type becomes ordinary Vigenère. Quagmire III uses the same keyed alphabet on both sides. A crib can therefore fix the shifts but not the alphabet: changing the alphabet moves plaintext and ciphertext letters together."</Note>
        </div>
    }
}
