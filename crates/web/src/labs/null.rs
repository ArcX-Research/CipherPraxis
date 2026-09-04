use super::widgets::{bars_svg, Note, NumberField, TextField};
use leptos::prelude::*;
use praxis_core::crypto::alphabet::normalize;
use praxis_core::crypto::stats::{periodic_ic, shuffled_null};

const SAMPLE: &str = "LXFOPVEFRNHRTRKMSIVVHTSDXWNOWNHXOQKHXRWGPDGVIBEGYYIBFGSYFJHEQWQCHBOJLRSSSMDFOWTTPLWQUAPTHAWEOMWQHXKEGCXXFSDMHIGSGLEPKTHVAYTXCOJXWXLZVMYBWMOQTVCXHEDWYABTWFXOKZTHSEDXBBZHMOZWAVIKFXWNOFPDFLPBHXOQKTZTUEAMIHKZFKQYOQQ";

#[component]
pub fn NullLab() -> impl IntoView {
    let text = RwSignal::new(SAMPLE.to_string());
    let period = RwSignal::new(5i64);
    let trials = RwSignal::new(300i64);
    let seed = RwSignal::new(1i64);
    let summary = Memo::new(move |_| {
        let m = period.get().max(1) as usize;
        shuffled_null(
            &normalize(&text.get()),
            trials.get().clamp(10, 5000) as usize,
            seed.get() as u64,
            move |t| periodic_ic(t, m),
        )
    });
    let sweep = Memo::new(move |_| {
        let t = normalize(&text.get());
        (2..=12)
            .map(|m| {
                let s = shuffled_null(&t, 120, seed.get() as u64, move |x| periodic_ic(x, m));
                (m.to_string(), s.z)
            })
            .collect::<Vec<_>>()
    });
    view! {
        <div class="lab">
            <div class="lab-controls">
                <TextField label="Ciphertext" value=text multiline=true mono=true/>
                <div class="lab-row">
                    <NumberField label="Period m to test" value=period min=1 max=40/>
                    <NumberField label="Number of shuffles" value=trials min=10 max=5000/>
                    <NumberField label="Seed" value=seed min=0 max=1000000/>
                </div>
            </div>
            <div class="stat-row">
                <div class="stat"><div class="stat-label mono">"observed mean coset IC"</div><div class="stat-value mono">{move || format!("{:.4}", summary.get().observed)}</div></div>
                <div class="stat"><div class="stat-label mono">"null mean ± standard deviation"</div><div class="stat-value mono">{move || format!("{:.4} ± {:.4}", summary.get().mean, summary.get().sd)}</div></div>
                <div class="stat"><div class="stat-label mono">"z"</div><div class="stat-value mono" class:hot=move || { summary.get().z > 3.0 }>{move || format!("{:+.2}", summary.get().z)}</div></div>
                <div class="stat"><div class="stat-label mono">"empirical upper-tail p-value"</div><div class="stat-value mono">{move || format!("{:.3}", summary.get().p_upper)}</div><div class="stat-sub mono">{move || format!("{} trials", summary.get().trials)}</div></div>
            </div>
            <div class="lab-viz">
                <div class="lab-viz-head mono">"Mean coset IC z-score versus 120 shuffles, for m = 2…12"</div>
                {move || { let d = sweep.get(); let hl: Vec<usize> = d.iter().enumerate().filter(|(_, (_, z))| *z > 3.0).map(|(i, _)| i).collect(); let data: Vec<(String, f64)> = d.into_iter().map(|(m, z)| (m, z.max(0.0))).collect(); view! { <div inner_html=bars_svg(&data, &hl, Some((3.0, "z = 3")))></div> } }}
            </div>
            <Note>"Shuffling removes position patterns but keeps the same letter counts. The whole-text IC therefore stays the same, while order-based statistics change. A large z-score gives evidence for a period under this null model. If you scan many periods and keep only the best, repeat the full scan on every shuffle so the p-value includes that choice."</Note>
        </div>
    }
}
