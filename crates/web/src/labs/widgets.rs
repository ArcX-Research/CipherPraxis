//! Shared lab widgets: labelled inputs, mono output blocks, histogram and bar charts (SVG).
use crate::util::group5;
use leptos::prelude::*;

#[component]
pub fn TextField(
    label: &'static str,
    value: RwSignal<String>,
    #[prop(optional)] hint: &'static str,
    #[prop(default = false)] multiline: bool,
    #[prop(default = false)] mono: bool,
) -> impl IntoView {
    let id = format!("f-{}", label.to_lowercase().replace(' ', "-"));
    let id2 = id.clone();
    let hint_id = format!("{id}-hint");
    let described_by = (!hint.is_empty()).then_some(hint_id.clone());
    view! {
        <div class="field">
            <label class="field-label" for=id>{label}</label>
            {if multiline {
                view! { <textarea id=id2 class="field-input" class:mono=mono rows="3" aria-describedby=described_by prop:value=move || value.get() on:input=move |ev| value.set(event_target_value(&ev))></textarea> }.into_any()
            } else {
                view! { <input id=id2 class="field-input" class:mono=mono type="text" aria-describedby=described_by prop:value=move || value.get() on:input=move |ev| value.set(event_target_value(&ev))/> }.into_any()
            }}
            {(!hint.is_empty()).then(|| view! { <span class="field-hint" id=hint_id>{hint}</span> })}
        </div>
    }
}

#[component]
pub fn NumberField(
    label: &'static str,
    value: RwSignal<i64>,
    #[prop(default = 0)] min: i64,
    #[prop(default = 1000)] max: i64,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let id = format!("n-{}", label.to_lowercase().replace(' ', "-"));
    let id2 = id.clone();
    view! {
        <div class="field field-num">
            <label class="field-label" for=id>{label}</label>
            <input id=id2 class="field-input mono" type="number" min=min max=max prop:value=move || value.get().to_string() on:input=move |ev| { if let Ok(v) = event_target_value(&ev).parse::<i64>() { value.set(v.clamp(min, max)); } }/>
            {children.map(|children| view! { <div class="field-extra">{children()}</div> })}
        </div>
    }
}

#[component]
pub fn Output(
    label: &'static str,
    #[prop(into)] value: Signal<String>,
    #[prop(default = true)] grouped: bool,
    #[prop(optional)] formula: &'static str,
) -> impl IntoView {
    view! {
        <div class="output">
            <div class="output-label">{label}<span class="output-len mono">{move || {
                let n = value.get().chars().filter(|c| c.is_ascii_alphabetic()).count();
                format!("{n} {}", if n == 1 { "letter" } else { "letters" })
            }}</span></div>
            {(!formula.is_empty()).then(|| view! { <div class="output-formula mono">{formula}</div> })}
            <div class="output-value mono">{move || {
                let text = value.get();
                if text.is_empty() { "—".into() } else if grouped { group5(&text) } else { text }
            }}</div>
        </div>
    }
}

#[component]
pub fn Note(children: Children) -> impl IntoView {
    view! { <p class="lab-note">{children()}</p> }
}

#[component]
pub fn ErrorNote(#[prop(into)] message: Signal<Option<String>>) -> impl IntoView {
    view! { {move || message.get().map(|m| view! { <p class="lab-error" role="alert">{m}</p> })} }
}

/// Horizontal bar chart of (label, value) pairs; `highlight` marks bars to accent.
pub fn bars_svg(
    data: &[(String, f64)],
    highlight: &[usize],
    reference: Option<(f64, &str)>,
) -> String {
    let n = data.len().max(1);
    let max = data
        .iter()
        .map(|d| d.1)
        .fold(0.0_f64, f64::max)
        .max(reference.map(|r| r.0).unwrap_or(0.0))
        .max(1e-9);
    let bw = 14.0;
    let gap = 4.0;
    let w = n as f64 * (bw + gap) + 36.0;
    let h = 120.0;
    let mut s = format!(
        r##"<svg class="chart" viewBox="0 0 {w:.0} {h:.0}" width="100%" role="img" aria-label="Bar chart">"##
    );
    if let Some((v, label)) = reference {
        let y = h - 20.0 - (v / max) * (h - 30.0);
        s.push_str(&format!(r##"<line x1="30" x2="{:.1}" y1="{y:.1}" y2="{y:.1}" stroke="var(--ink)" stroke-dasharray="3 3"/><text x="{:.1}" y="{:.1}" font-size="8" fill="var(--ink-2)" text-anchor="end" font-family="DM Mono, monospace">{label}</text>"##, w - 4.0, w - 4.0, y - 3.0));
    }
    for (i, (label, v)) in data.iter().enumerate() {
        let x = 32.0 + i as f64 * (bw + gap);
        let bh = (v / max) * (h - 30.0);
        let y = h - 20.0 - bh;
        let fill = if highlight.contains(&i) {
            "var(--blue)"
        } else {
            "var(--chart-bar)"
        };
        s.push_str(&format!(r##"<rect x="{x:.1}" y="{y:.1}" width="{bw}" height="{bh:.1}" rx="2" fill="{fill}"><title>{label}: {v:.4}</title></rect><text x="{:.1}" y="{:.1}" font-size="8" text-anchor="middle" fill="var(--ink-2)" font-family="DM Mono, monospace">{label}</text>"##, x + bw / 2.0, h - 8.0));
    }
    s.push_str("</svg>");
    s
}

/// Letter histogram (A–Z) as an SVG.
pub fn histogram_svg(counts: &[usize; 26], title: &str) -> String {
    let data: Vec<(String, f64)> = counts
        .iter()
        .enumerate()
        .map(|(i, &c)| (((b'A' + i as u8) as char).to_string(), c as f64))
        .collect();
    let max_i = counts
        .iter()
        .enumerate()
        .max_by_key(|(_, c)| **c)
        .map(|(i, _)| i)
        .unwrap_or(0);
    let mut s = bars_svg(&data, &[max_i], None);
    s.insert_str(
        s.find('>').map(|i| i + 1).unwrap_or(0),
        &format!(r##"<title>{title}</title>"##),
    );
    s
}
