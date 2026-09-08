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
