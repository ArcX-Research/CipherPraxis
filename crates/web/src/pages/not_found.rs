use crate::util::set_title;
use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    set_title("Not found");
    view! {
        <section class="wrap page-narrow">
            <p class="mono meta">"404"</p>
            <h1 class="display">"Page not found."</h1>
            <p class="lede">"This link may be old or mistyped. Search for the page, or return to the overview."</p>
            <p><a class="btn btn-primary" href="/">"Go to the overview"</a></p>
        </section>
    }
}
