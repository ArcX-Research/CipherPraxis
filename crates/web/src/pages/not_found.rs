use crate::util::set_title;
use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    set_title("Not found");
    view! {
        <section class="wrap page-narrow">
            <p class="mono meta">"404"</p>
            <h1 class="display">"Nothing at this address."</h1>
            <p class="lede">"The page may have moved when the knowledge base was reorganised. Try the search, or start from the overview."</p>
            <p><a class="btn btn-primary" href="/">"Back to the overview"</a></p>
        </section>
    }
}
