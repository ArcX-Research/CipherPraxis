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

/// Cipher disc: an outer plaintext ring and a keyed inner ciphertext ring that turns slowly
/// under a fixed spectral pointer. Pure SVG; the motion is a CSS animation on `.disc-inner`
/// (disabled automatically under `prefers-reduced-motion`).
pub fn cipher_disc_svg() -> String {
    use praxis_core::crypto::alphabet::Alphabet;
    use std::f64::consts::PI;
    let c = 200.0_f64;
    let outer = Alphabet::straight().to_string();
    let inner = Alphabet::keyword("CIPHERPRAXIS").to_string();
    let mut s = String::with_capacity(12_000);
    s.push_str(r##"<svg class="cipher-disc" viewBox="-44 -44 488 488" width="100%" role="img" aria-label="Cipher disc: an outer plaintext ring and a rotating keyed inner ring">
<defs>
<linearGradient id="cd-spectral" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#0454ff"/><stop offset="0.55" stop-color="#6764ff"/><stop offset="1" stop-color="#a95dc9"/></linearGradient>
<radialGradient id="cd-glow" cx="0.5" cy="0.5" r="0.5"><stop offset="0" stop-color="#e1e0ff" stop-opacity="0.85"/><stop offset="0.55" stop-color="#ebe8ff" stop-opacity="0.35"/><stop offset="1" stop-color="#ffffff" stop-opacity="0"/></radialGradient>
<radialGradient id="cd-hub" cx="0.5" cy="0.5" r="0.5"><stop offset="0" stop-color="#ffffff"/><stop offset="1" stop-color="#f3f1ff"/></radialGradient>
</defs>
<circle cx="200" cy="200" r="240" fill="url(#cd-glow)"/>
"##);
    // Rings.
    for (r, op, w) in [
        (190.0, 0.22, 1.0),
        (152.0, 0.16, 1.0),
        (140.0, 0.22, 1.0),
        (104.0, 0.16, 1.0),
        (78.0, 0.28, 1.2),
    ] {
        s.push_str(&format!(r##"<circle cx="200" cy="200" r="{r}" fill="none" stroke="#000020" stroke-opacity="{op}" stroke-width="{w}"/>"##));
    }
    // Outer decorative tick ring (52 ticks, spectral, turns very slowly).
    s.push_str(r##"<g class="disc-ticks" stroke="url(#cd-spectral)" stroke-width="1.2" stroke-linecap="round" opacity="0.7">"##);
    for i in 0..52 {
        let a = i as f64 * 2.0 * PI / 52.0 - PI / 2.0;
        let (r1, r2) = if i % 2 == 0 {
            (193.0, 199.0)
        } else {
            (195.0, 198.0)
        };
        s.push_str(&format!(
            r##"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}"/>"##,
            c + r1 * a.cos(),
            c + r1 * a.sin(),
            c + r2 * a.cos(),
            c + r2 * a.sin()
        ));
    }
    s.push_str("</g>");
    // Outer plaintext ring (fixed).
    s.push_str(r##"<g font-family="DM Mono, Menlo, monospace" font-size="15" text-anchor="middle" fill="#000020" fill-opacity="0.78">"##);
    for (i, ch) in outer.chars().enumerate() {
        let a = i as f64 * 2.0 * PI / 26.0 - PI / 2.0;
        let (x, y) = (c + 171.0 * a.cos(), c + 171.0 * a.sin());
        let deg = i as f64 * 360.0 / 26.0;
        s.push_str(&format!(r##"<text x="{x:.2}" y="{y:.2}" transform="rotate({deg:.2} {x:.2} {y:.2})" dy="5">{ch}</text>"##));
        let (t1, t2) = (c + 146.0 * a.cos(), c + 146.0 * a.sin());
        let (u1, u2) = (c + 150.0 * a.cos(), c + 150.0 * a.sin());
        s.push_str(&format!(r##"<line x1="{t1:.2}" y1="{t2:.2}" x2="{u1:.2}" y2="{u2:.2}" stroke="#000020" stroke-opacity="0.35" stroke-width="1"/>"##));
    }
    s.push_str("</g>");
    // Inner keyed ring (rotates).
    s.push_str(r##"<g class="disc-inner"><circle cx="200" cy="200" r="122" fill="none" stroke="#000020" stroke-opacity="0.08" stroke-width="30"/><g font-family="DM Mono, Menlo, monospace" font-size="15" font-weight="500" text-anchor="middle" fill="#0a3fbf">"##);
    for (i, ch) in inner.chars().enumerate() {
        let a = i as f64 * 2.0 * PI / 26.0 - PI / 2.0;
        let (x, y) = (c + 122.0 * a.cos(), c + 122.0 * a.sin());
        let deg = i as f64 * 360.0 / 26.0;
        s.push_str(&format!(r##"<text x="{x:.2}" y="{y:.2}" transform="rotate({deg:.2} {x:.2} {y:.2})" dy="5">{ch}</text>"##));
    }
    s.push_str("</g></g>");
    // Fixed pointer at twelve o'clock: a lens over the inner letter and a marker on the outer A.
    s.push_str(r##"<rect x="185" y="14" width="30" height="30" rx="8" fill="none" stroke="url(#cd-spectral)" stroke-width="1.6"/>
<rect x="184" y="63" width="32" height="30" rx="8" fill="rgba(225,224,255,0.35)" stroke="url(#cd-spectral)" stroke-width="1.6"/>
<path d="M200 46 L200 60" stroke="url(#cd-spectral)" stroke-width="1.6" stroke-linecap="round"/>
<circle cx="200" cy="200" r="60" fill="url(#cd-hub)" stroke="url(#cd-spectral)" stroke-width="1.2"/>
<g stroke="url(#cd-spectral)" stroke-width="1" opacity="0.6"><path d="M178 190h44M178 200h44M178 210h44M190 178v44M200 178v44M210 178v44"/></g>
<circle cx="190" cy="190" r="2.2" fill="#0454ff"/><circle cx="200" cy="200" r="2.2" fill="#6764ff"/><circle cx="210" cy="210" r="2.2" fill="#a95dc9"/>
</svg>"##);
    s
}

#[component]
pub fn CipherDisc() -> impl IntoView {
    let svg = cipher_disc_svg();
    view! { <div class="cipher-disc-wrap" inner_html=svg></div> }
}
