use super::widgets::{Note, NumberField, Output, TextField};
use leptos::prelude::*;
use praxis_core::crypto::alphabet::normalize;
use praxis_core::crypto::bifid::{decrypt, encrypt, Polybius};

#[component]
pub fn BifidLab() -> impl IntoView {
    let plain = RwSignal::new("Flee at once we are discovered".to_string());
    let kw = RwSignal::new("PLAYFAIR".to_string());
    let period = RwSignal::new(5i64);
    let square = Memo::new(move |_| Polybius::from_keyword(&kw.get()));
    let cipher =
        Signal::derive(move || encrypt(&plain.get(), &square.get(), period.get().max(0) as usize));
    let back =
        Signal::derive(move || decrypt(&cipher.get(), &square.get(), period.get().max(0) as usize));
    view! {
        <div class="lab">
            <div class="lab-controls">
                <TextField label="Plaintext" value=plain multiline=true/>
                <div class="lab-row">
                    <TextField label="Square keyword" value=kw mono=true hint="J is treated as I"/>
                    <NumberField label="Period (use 0 for all text)" value=period min=0 max=100/>
                </div>
            </div>
            <div class="lab-viz">
                <div class="lab-viz-head mono">"Polybius square · coordinates (row, column)"</div>
                <div class="lab-row">
                    <div class="grid-table mono square">
                        <div class="grid-row grid-head"><span></span>{(1..=5).map(|c| view! { <span><b>{c}</b></span> }).collect_view()}</div>
                        {move || square.get().rows().into_iter().enumerate().map(|(r, row)| view! { <div class="grid-row"><span><b>{r + 1}</b></span>{row.chars().map(|c| view! { <span>{c.to_string()}</span> }).collect_view()}</div> }).collect_view()}
                    </div>
                    <div class="coords mono">
                        {move || {
                            let t = normalize(&plain.get()).replace('J', "I");
                            let p = period.get().max(0) as usize;
                            let p = if p == 0 { t.len().max(1) } else { p };
                            let sq = square.get();
                            t.as_bytes().chunks(p).take(3).map(|chunk| {
                                let (rows, cols): (Vec<u8>, Vec<u8>) = chunk.iter().map(|&b| sq.coords(b)).unzip();
                                view! {
                                    <div class="coord-block">
                                        <div>{chunk.iter().map(|&b| view! { <i>{(b as char).to_string()}</i> }).collect_view()}</div>
                                        <div>{rows.iter().map(|r| view! { <i>{r.to_string()}</i> }).collect_view()}</div>
                                        <div>{cols.iter().map(|c| view! { <i>{c.to_string()}</i> }).collect_view()}</div>
                                    </div>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>
            </div>
            <Output label="Ciphertext" value=cipher/>
            <Output label="Decrypted" value=back/>
            <Note>"For each block, Bifid writes all row numbers and then all column numbers. It pairs that new number stream back into letters. One ciphertext letter can therefore depend on two plaintext letters up to one period apart. This hides simple letter frequencies, so an attack usually finds the period first."</Note>
        </div>
    }
}
