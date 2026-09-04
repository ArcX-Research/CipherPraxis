use super::widgets::{Note, Output, TextField};
use leptos::prelude::*;
use praxis_core::crypto::hill::{decrypt, encrypt, Matrix};

#[component]
pub fn HillLab() -> impl IntoView {
    let plain = RwSignal::new("Help".to_string());
    let cells = RwSignal::new("3 3 2 5".to_string());
    let size = RwSignal::new(2usize);
    let matrix = Memo::new(move |_| {
        let vals: Result<Vec<i64>, _> = cells
            .get()
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>())
            .collect();
        match vals {
            Ok(v) => Matrix::new(size.get(), v),
            Err(_) => Err("matrix entries must be integers".to_string()),
        }
    });
    let cipher = Signal::derive(move || {
        matrix
            .get()
            .map(|m| encrypt(&plain.get(), &m))
            .unwrap_or_default()
    });
    let back = Signal::derive(move || {
        matrix
            .get()
            .ok()
            .and_then(|m| decrypt(&cipher.get(), &m).ok())
            .unwrap_or_default()
    });
    let err = Signal::derive(move || matrix.get().err());
    view! {
        <div class="lab">
            <div class="lab-controls">
                <TextField label="Plaintext" value=plain/>
                <div class="lab-row">
                    <div class="field">
                        <label class="field-label" for="hsize">"Block size"</label>
                        <select id="hsize" class="field-input" on:change=move |ev| { let n = event_target_value(&ev).parse().unwrap_or(2); size.set(n); cells.set(if n == 2 { "3 3 2 5".into() } else { "6 24 1 13 16 10 20 17 15".into() }); }>
                            <option value="2" selected=move || size.get() == 2>"2 × 2"</option>
                            <option value="3" selected=move || size.get() == 3>"3 × 3"</option>
                        </select>
                    </div>
                    <TextField label="Key matrix (row-major)" value=cells mono=true hint="entries mod 26"/>
                </div>
            </div>
            {move || err.get().map(|e| view! { <p class="lab-error" role="alert">{e}</p> })}
            {move || matrix.get().ok().map(|m| {
                let det = m.det();
                let inv = m.inverse();
                view! {
                    <div class="lab-viz">
                        <div class="lab-viz-head mono">{format!("det K ≡ {det} (mod 26) · {}", if inv.is_some() { "invertible: gcd(det, 26) = 1" } else { "NOT invertible: gcd(det, 26) ≠ 1 — decryption impossible" })}</div>
                        <div class="matrix-row">
                            <MatrixView label="K" m=m.clone()/>
                            {inv.map(|i| view! { <MatrixView label="K⁻¹" m=i/> })}
                        </div>
                    </div>
                }
            })}
            <Output label="Ciphertext (blocks padded with X)" value=cipher/>
            <Output label="Decrypted" value=back/>
            <Note>"A Hill key is a linear map on n-letter blocks; it is breakable with n² known plaintext letters because the key satisfies a linear system over Z26. Try determinant 13 or an even determinant to see the invertibility condition bite."</Note>
        </div>
    }
}

#[component]
fn MatrixView(label: &'static str, m: Matrix) -> impl IntoView {
    let n = m.n;
    view! {
        <div class="matrix mono">
            <div class="matrix-label">{label}</div>
            <div class="matrix-body" style=format!("grid-template-columns: repeat({n}, 1fr)")>
                {m.data.iter().map(|v| view! { <span>{*v}</span> }).collect_view()}
            </div>
        </div>
    }
}
