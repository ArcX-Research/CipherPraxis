use super::widgets::{bars_svg, Note, TextField};
use leptos::prelude::*;
use praxis_core::crypto::alphabet::normalize;
use praxis_core::crypto::stats::{
    english_ic, factor_votes, ic_scan, ic_std_dev, index_of_coincidence, kasiski, lag_profile,
    ENGLISH, UNIFORM_IC,
};

const SAMPLE: &str = "LXFOPVEFRNHRTRKMSIVVHTSDXWNOWNHXOQKHXRWGPDGVIBEGYYIBFGSYFJHEQWQCHBOJLRSSSMDFOWTTPLWQUAPTHAWEOMWQHXKEGCXXFSDMHIGSGLEPKTHVAYTXCOJXWXLZVMYBWMOQTVCXHEDWYABTWFXOKZTHSEDXBBZHMOZWAVIKFXWNOFPDFLPBHXOQKTZTUEAMIHKZFKQYOQQ";

#[component]
pub fn CoincidenceLab() -> impl IntoView {
    let text = RwSignal::new(SAMPLE.to_string());
    let norm = Memo::new(move |_| normalize(&text.get()));
    let n = Memo::new(move |_| norm.get().len());
    let ic = Memo::new(move |_| index_of_coincidence(&norm.get()));
    let scan = Memo::new(move |_| ic_scan(&norm.get(), 15));
    let lags = Memo::new(move |_| lag_profile(&norm.get(), 24));
    let reps = Memo::new(move |_| kasiski(&norm.get(), 3, 6));
    let votes = Memo::new(move |_| factor_votes(&reps.get(), 15));
    view! {
        <div class="lab">
            <div class="lab-controls">
                <TextField label="Ciphertext" value=text multiline=true mono=true hint="paste any text; non-letters are ignored"/>
            </div>
            <div class="stat-row">
                <div class="stat"><div class="stat-label mono">"letters"</div><div class="stat-value mono">{move || n.get()}</div></div>
                <div class="stat"><div class="stat-label mono">"IC (whole text)"</div><div class="stat-value mono">{move || format!("{:.4}", ic.get())}</div></div>
                <div class="stat"><div class="stat-label mono">"English reference"</div><div class="stat-value mono">{format!("{:.4} ± {:.4}", english_ic(), 0.0)}</div><div class="stat-sub mono">{move || format!("s.d. at n={}: {:.4}", n.get(), ic_std_dev(&ENGLISH, n.get()))}</div></div>
                <div class="stat"><div class="stat-label mono">"uniform reference"</div><div class="stat-value mono">{format!("{UNIFORM_IC:.4}")}</div></div>
            </div>
            <div class="lab-viz">
                <div class="lab-viz-head mono">"Mean coset IC by trial period m = 1…15 · dashed line = English IC"</div>
                {move || {
                    let data: Vec<(String, f64)> = scan.get().into_iter().map(|(m, v)| (m.to_string(), v)).collect();
                    let best = scan.get().iter().skip(1).enumerate().max_by(|a, b| a.1 .1.partial_cmp(&b.1 .1).unwrap()).map(|(i, _)| i + 1);
                    let hl: Vec<usize> = best.into_iter().collect();
                    view! { <div inner_html=bars_svg(&data, &hl, Some((english_ic(), "English")))></div> }
                }}
            </div>
            <div class="lab-viz">
                <div class="lab-viz-head mono">"Coincidence rate at lag t = 1…24 · fraction of positions with cᵢ = cᵢ₊ₜ"</div>
                {move || {
                    let data: Vec<(String, f64)> = lags.get().into_iter().map(|(l, v)| (l.to_string(), v)).collect();
                    let top = lags.get().iter().enumerate().max_by(|a, b| a.1 .1.partial_cmp(&b.1 .1).unwrap()).map(|(i, _)| i);
                    view! { <div inner_html=bars_svg(&data, &top.into_iter().collect::<Vec<_>>(), Some((english_ic(), "English")))></div> }
                }}
            </div>
            <div class="lab-viz">
                <div class="lab-viz-head mono">"Kasiski examination · repeated 3–6-grams and the periods dividing their distances"</div>
                <div class="kasiski">
                    <div class="table-scroll">
                        <table class="mini-table mono">
                            <thead><tr><th>"gram"</th><th>"positions"</th><th>"distances"</th></tr></thead>
                            <tbody>
                                {move || reps.get().into_iter().take(10).map(|r| view! { <tr><td>{r.gram}</td><td>{r.positions.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")}</td><td>{r.distances.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(", ")}</td></tr> }).collect_view()}
                            </tbody>
                        </table>
                    </div>
                    {move || {
                        let data: Vec<(String, f64)> = votes.get().into_iter().map(|(p, v)| (p.to_string(), v as f64)).collect();
                        let top = votes.get().iter().enumerate().max_by_key(|(_, (_, v))| *v).map(|(i, _)| i);
                        view! { <div inner_html=bars_svg(&data, &top.into_iter().collect::<Vec<_>>(), None)></div> }
                    }}
                </div>
            </div>
            <Note>"The sample is a Vigenère ciphertext with a five-letter key. Period 5 (and its multiple 10) stand out in all three instruments; on a short text the whole-text IC alone says little, which is why the exact standard deviation of the estimator is printed next to it."</Note>
        </div>
    }
}
