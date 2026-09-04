//! Renders Markdown+math content bodies. Internal links are ordinary anchors; the router
//! intercepts them for client-side navigation.
use leptos::prelude::*;
use praxis_core::render::markdown_to_html;

#[component]
pub fn Markdown(
    #[prop(into)] source: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let html = markdown_to_html(&source);
    view! { <div class=format!("prose {class}") inner_html=html></div> }
}
