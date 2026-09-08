use super::widgets::{ErrorNote, Note, Output, TextField};
use leptos::prelude::*;
use praxis_core::crypto::alphabet::normalize;
use praxis_core::crypto::transposition::{columnar_decrypt, columnar_encrypt, columns, key_order};

#[component]
pub fn ColumnarLab() -> impl IntoView {
    let plain = RwSignal::new("We are discovered flee at once".to_string());
    let key = RwSignal::new("ZEBRAS".to_string());
    let padded = RwSignal::new(false);
    let res = Memo::new(move |_| {
        columnar_encrypt(
            &plain.get(),
            &key.get(),
            if padded.get() { Some('X') } else { None },
        )
    });
    let cipher = Signal::derive(move || res.get().unwrap_or_default());
    let err = Signal::derive(move || res.get().err());
    let back = Signal::derive(move || {
        res.get()
            .ok()
            .and_then(|c| columnar_decrypt(&c, &key.get()).ok())
            .unwrap_or_default()
    });
    view! {
        <div class="lab">
            <div class="lab-controls">
                <TextField label="Plaintext" value=plain multiline=true/>
                <div class="lab-fields">
                    <TextField label="Keyword" value=key mono=true/>
                    <div class="field">
                        <span class="field-label">"Padding"</span>
                        <label class="field-toggle filter-toggle"><input type="checkbox" prop:checked=move || padded.get() on:change=move |ev| padded.set(event_target_checked(&ev))/><span>"Fill the last row with padding"</span></label>
                    </div>
                </div>
            </div>
            <ErrorNote message=err/>
            <div class="lab-viz">
                <div class="lab-viz-head mono">"Grid · read columns in keyword order"</div>
                {move || {
                    let k = normalize(&key.get());
                    let order = key_order(&k);
                    let mut rank = vec![0usize; order.len()];
                    for (r, &c) in order.iter().enumerate() { rank[c] = r + 1; }
                    let t = if padded.get() { let mut t = normalize(&plain.get()); while !k.is_empty() && t.len() % k.len() != 0 { t.push('X'); } t } else { normalize(&plain.get()) };
                    let cols = columns(&t, k.len().max(1));
                    let rows = cols.iter().map(|c| c.len()).max().unwrap_or(0);
                    view! {
                        <div class="grid-table mono">
                            <div class="grid-row grid-head">{k.chars().enumerate().map(|(i, c)| view! { <span><b>{c.to_string()}</b><small>{rank.get(i).map(|r| r.to_string()).unwrap_or_default()}</small></span> }).collect_view()}</div>
                            {(0..rows).map(|r| view! { <div class="grid-row">{cols.iter().map(|c| view! { <span>{c.chars().nth(r).map(|x| x.to_string()).unwrap_or_else(|| "·".into())}</span> }).collect_view()}</div> }).collect_view()}
                        </div>
                    }
                }}
            </div>
            <Output label="Ciphertext" value=cipher/>
            <Output label="Decrypted" value=back/>
            <Note>"Without padding, the last row is short and the leftmost columns hold one extra letter. During decryption, the text length tells you which columns are longer. An order-free width test uses this long-and-short pattern to estimate the grid width without knowing the keyword."</Note>
        </div>
    }
}
