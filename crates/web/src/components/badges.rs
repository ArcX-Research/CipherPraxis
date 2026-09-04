//! Status badges, kind chips and tag chips.
use leptos::prelude::*;
use praxis_core::content::Status;

#[component]
pub fn StatusBadge(status: Status, #[prop(optional)] large: bool) -> impl IntoView {
    view! {
        <span
            class=format!("status status-{}", status.slug())
            class:status-lg=large
            title=status.description()
        >
            <i class="status-dot" aria-hidden="true"></i>
            {status.label()}
        </span>
    }
}
