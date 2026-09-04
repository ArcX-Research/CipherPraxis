use super::widgets::{ErrorNote, Note, NumberField, Output, TextField};
use leptos::prelude::*;
use praxis_core::crypto::affine::{decrypt, encrypt};
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
    view! {
        <div class="lab">
            <div class="lab-controls">
                <TextField label="Plaintext" value=plain/>
                <div class="lab-row">
                    <NumberField label="a (multiplier)" value=a min=0 max=25/>
                    <NumberField label="b (shift)" value=b min=0 max=25/>
                </div>
            </div>
            <ErrorNote message=err/>
            <Output label="Ciphertext c = a·p + b (mod 26)" value=cipher/>
            <Output label="Decrypted p = a⁻¹(c − b) (mod 26)" value=back/>
            <div class="lab-viz">
                <div class="lab-viz-head mono">{move || match inv.get() { Some(i) => format!("a⁻¹ = {i} because {}·{i} = {} ≡ 1 (mod 26)", a.get(), a.get() * i), None => format!("{} has no inverse: gcd({}, 26) ≠ 1", a.get(), a.get()) }}</div>
                <div class="unit-row mono">
                    {units(26).into_iter().map(|u| view! { <button type="button" class="unit" class:active=move || a.get() == u on:click=move |_| a.set(u)>{u}<span class="unit-inv">{format!("⁻¹={}", mod_inverse(u, 26).unwrap())}</span></button> }).collect_view()}
                </div>
            </div>
            <Note>"Only the 12 units of Z26 can be multipliers. A unit is a number that has an inverse modulo 26. This gives 12 × 26 = 312 affine keys, so an attack can try every key."</Note>
        </div>
    }
}
