//! Brand mark and geometric motifs (static SVG strings rendered through `inner_html`).
use leptos::prelude::*;

pub const ICON_SEARCH: &str = r##"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><circle cx="7" cy="7" r="4.5"/><path d="M10.5 10.5 14 14"/></svg>"##;

const LOGO: &str = r##"<svg class="logo" width="30" height="30" viewBox="0 0 32 32" fill="none" aria-hidden="true">
<defs><linearGradient id="cp-spectral" x1="0" y1="0" x2="32" y2="32" gradientUnits="userSpaceOnUse"><stop offset="0" stop-color="#0454ff"/><stop offset="0.55" stop-color="#6764ff"/><stop offset="1" stop-color="#a95dc9"/></linearGradient></defs>
<rect x="1.5" y="1.5" width="29" height="29" rx="8" stroke="url(#cp-spectral)" stroke-width="1.5"/>
<g stroke="url(#cp-spectral)" stroke-width="1" opacity="0.55"><path d="M8 11.5h16M8 16h16M8 20.5h16"/><path d="M11.5 8v16M16 8v16M20.5 8v16"/></g>
<circle cx="11.5" cy="11.5" r="1.6" fill="#0454ff"/><circle cx="16" cy="16" r="1.6" fill="#6764ff"/><circle cx="20.5" cy="20.5" r="1.6" fill="#a95dc9"/>
</svg>"##;

#[component]
pub fn Logo() -> impl IntoView {
    view! { <span class="logo-wrap" inner_html=LOGO></span> }
}

/// Animated "cipher field": a fragment of a tabula recta with a drifting spectral highlight.
pub fn cipher_field_svg(rows: usize, cols: usize) -> String {
    let mut s = String::new();
    let cell = 26.0;
    let w = cols as f64 * cell;
    let h = rows as f64 * cell;
    s.push_str(&format!(
        r##"<svg class="cipher-field" viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="Tabula recta grid with each alphabet row shifted by one letter">"##
    ));
    s.push_str(r##"<defs><linearGradient id="cf-spectral" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#0454ff"/><stop offset="0.5" stop-color="#6764ff"/><stop offset="1" stop-color="#a95dc9"/></linearGradient>
<radialGradient id="cf-glow" cx="0.5" cy="0.5" r="0.5"><stop offset="0" stop-color="#e1e0ff" stop-opacity="0.9"/><stop offset="1" stop-color="#ffffff" stop-opacity="0"/></radialGradient></defs>"##);
    s.push_str(&format!(
        r##"<rect class="cf-glow" width="{w}" height="{h}" fill="url(#cf-glow)"/>"##,
        w = w * 0.9,
        h = h * 0.9
    ));
    for r in 0..=rows {
        let y = r as f64 * cell;
        s.push_str(&format!(
            r##"<line x1="0" y1="{y}" x2="{w}" y2="{y}" stroke="#000020" stroke-opacity="0.08"/>"##
        ));
    }
    for c in 0..=cols {
        let x = c as f64 * cell;
        s.push_str(&format!(
            r##"<line x1="{x}" y1="0" x2="{x}" y2="{h}" stroke="#000020" stroke-opacity="0.08"/>"##
        ));
    }
    for r in 0..rows {
        for c in 0..cols {
            let ch = (b'A' + ((r + c) % 26) as u8) as char;
            let x = c as f64 * cell + cell / 2.0;
            let y = r as f64 * cell + cell / 2.0 + 4.5;
            let on_diag = r == c;
            let fill = if on_diag {
                "url(#cf-spectral)"
            } else {
                "#000020"
            };
            let op = if on_diag { 1.0 } else { 0.42 };
            s.push_str(&format!(
                r##"<text x="{x:.1}" y="{y:.1}" text-anchor="middle" font-family="DM Mono, Menlo, monospace" font-size="12" fill="{fill}" fill-opacity="{op}">{ch}</text>"##
            ));
        }
    }
    s.push_str(&format!(
        r##"<rect class="cf-cursor" x="0" y="0" width="{cell}" height="{cell}" rx="4" fill="none" stroke="url(#cf-spectral)" stroke-width="1.5"/>"##
    ));
    s.push_str("</svg>");
    s
}

#[component]
pub fn CipherField(
    #[prop(default = 9)] rows: usize,
    #[prop(default = 13)] cols: usize,
) -> impl IntoView {
    let svg = cipher_field_svg(rows, cols);
    view! { <div class="cipher-field-wrap" inner_html=svg></div> }
}
