//! Evidence-request form. The site is static, so delivery is configurable in `state.rs`:
//! a JSON endpoint (`EVIDENCE_REQUEST_ENDPOINT`), a mailbox (`EVIDENCE_REQUEST_EMAIL`), or,
//! with neither set, a request text the visitor can copy and send.
use crate::state::{EVIDENCE_REQUEST_EMAIL, EVIDENCE_REQUEST_ENDPOINT};
use leptos::prelude::*;
use leptos::task::spawn_local;
use praxis_core::content::Provenance;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, PartialEq)]
enum Status {
    Idle,
    Sending,
    Sent,
    Copied,
    Failed(String),
}

const PURPOSES: [(&str, &str); 4] = [
    ("research", "Research"),
    ("teaching", "Teaching"),
    ("verification", "Verification or audit"),
    ("other", "Other"),
];

fn valid_email(s: &str) -> bool {
    let s = s.trim();
    s.len() >= 5
        && s.contains('@')
        && s.rsplit('@')
            .next()
            .map(|d| d.contains('.'))
            .unwrap_or(false)
}

fn page_url() -> String {
    window().location().href().unwrap_or_default()
}

fn timestamp() -> String {
    String::from(js_sys::Date::new_0().to_iso_string())
}

/// Plain-text version of the request (used for e-mail bodies and the copy fallback).
fn request_text(
    entry_title: &str,
    entry_id: &str,
    files: &[String],
    name: &str,
    email: &str,
    purpose: &str,
    message: &str,
) -> String {
    let mut s = String::new();
    s.push_str(&format!("Evidence request — {entry_title} ({entry_id})\n"));
    s.push_str(&format!("Page: {}\n\n", page_url()));
    s.push_str("Files requested:\n");
    for f in files {
        s.push_str(&format!("  - {f}\n"));
    }
    s.push_str(&format!(
        "\nName: {name}\nEmail: {email}\nPurpose: {purpose}\n"
    ));
    if !message.trim().is_empty() {
        s.push_str(&format!("\nMessage:\n{}\n", message.trim()));
    }
    s.push_str(&format!("\nSubmitted: {}\n", timestamp()));
    s
}

fn request_json(
    entry_title: &str,
    entry_id: &str,
    files: &[String],
    name: &str,
    email: &str,
    purpose: &str,
    message: &str,
) -> String {
    serde_json::json!({
        "kind": "evidence-request",
        "entry": entry_id,
        "title": entry_title,
        "url": page_url(),
        "files": files,
        "name": name.trim(),
        "email": email.trim(),
        "purpose": purpose,
        "message": message.trim(),
        "submitted_at": timestamp(),
    })
    .to_string()
}

async fn post_json(endpoint: &str, body: String) -> Result<(), String> {
    let opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&wasm_bindgen::JsValue::from_str(&body));
    let request = web_sys::Request::new_with_str_and_init(endpoint, &opts)
        .map_err(|_| "The request could not be built.".to_string())?;
    request
        .headers()
        .set("Content-Type", "application/json")
        .map_err(|_| "The request headers could not be set.".to_string())?;
    let response = wasm_bindgen_futures::JsFuture::from(window().fetch_with_request(&request))
        .await
        .map_err(|_| {
            "The request could not be sent. Check your connection and try again.".to_string()
        })?;
    let response: web_sys::Response = response
        .dyn_into()
        .map_err(|_| "Unexpected reply from the server.".to_string())?;
    if response.ok() {
        Ok(())
    } else {
        Err(format!(
            "The server replied with status {}.",
            response.status()
        ))
    }
}

fn mailto(address: &str, subject: &str, body: &str) -> String {
    format!(
        "mailto:{address}?subject={}&body={}",
        js_sys::encode_uri_component(subject),
        js_sys::encode_uri_component(body)
    )
}

#[component]
pub fn EvidenceRequest(
    entry_id: String,
    entry_title: String,
    provenance: Vec<Provenance>,
) -> impl IntoView {
    // One line per file: the path, plus its log or ledger reference when there is one.
    let files: Vec<String> = provenance
        .iter()
        .map(|p| match &p.reference {
            Some(r) => format!("{} · {}", p.path, r),
            None => p.path.clone(),
        })
        .collect();
    let selected = RwSignal::new(vec![true; files.len()]);
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let purpose = RwSignal::new("research".to_string());
    let message = RwSignal::new(String::new());
    let status = RwSignal::new(Status::Idle);
    let error = RwSignal::new(Option::<String>::None);
    let output = RwSignal::new(Option::<String>::None);
    let mail_link = RwSignal::new(Option::<String>::None);

    let files_for_submit = files.clone();
    let id_for_submit = entry_id.clone();
    let title_for_submit = entry_title.clone();
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let chosen: Vec<String> = files_for_submit
            .iter()
            .zip(selected.get())
            .filter(|(_, on)| *on)
            .map(|(f, _)| f.clone())
            .collect();
        if name.get().trim().is_empty() {
            error.set(Some("Please enter your name.".into()));
            return;
        }
        if !valid_email(&email.get()) {
            error.set(Some("Please enter a valid email address.".into()));
            return;
        }
        if chosen.is_empty() {
            error.set(Some("Select at least one file.".into()));
            return;
        }
        error.set(None);
        let purpose_label = PURPOSES
            .iter()
            .find(|(k, _)| *k == purpose.get())
            .map(|(_, l)| *l)
            .unwrap_or("Other");
        let text = request_text(
            &title_for_submit,
            &id_for_submit,
            &chosen,
            &name.get(),
            &email.get(),
            purpose_label,
            &message.get(),
        );
        if let Some(endpoint) = EVIDENCE_REQUEST_ENDPOINT {
            let body = request_json(
                &title_for_submit,
                &id_for_submit,
                &chosen,
                &name.get(),
                &email.get(),
                &purpose.get(),
                &message.get(),
            );
            status.set(Status::Sending);
            spawn_local(async move {
                match post_json(endpoint, body).await {
                    Ok(()) => status.set(Status::Sent),
                    Err(e) => status.set(Status::Failed(e)),
                }
            });
        } else if let Some(address) = EVIDENCE_REQUEST_EMAIL {
            let subject = format!("Evidence request: {title_for_submit}");
            let link = mailto(address, &subject, &text);
            let _ = window().location().set_href(&link);
            mail_link.set(Some(link));
            output.set(Some(text));
            status.set(Status::Sent);
        } else {
            let _ = window().navigator().clipboard().write_text(&text);
            output.set(Some(text));
            status.set(Status::Copied);
        }
    };

    let file_rows = files.clone();
    view! {
        <details class="request">
            <summary>"Ask for these files"</summary>
            <p class="request-intro">
                "These files are internal. Choose the ones you need, tell us how you will use them, and the team will reply by email."
            </p>
            <form class="request-form" on:submit=submit novalidate>
                <fieldset class="request-files">
                    <legend>"Files"</legend>
                    {file_rows.into_iter().enumerate().map(|(i, f)| {
                        let id = format!("req-file-{i}");
                        let label_for = id.clone();
                        view! {
                            <label class="request-file" for=label_for>
                                <input
                                    id=id
                                    type="checkbox"
                                    prop:checked=move || selected.with(|s| s.get(i).copied().unwrap_or(false))
                                    on:change=move |ev| { let on = event_target_checked(&ev); selected.update(|s| { if let Some(slot) = s.get_mut(i) { *slot = on; } }); }
                                />
                                <span>{f}</span>
                            </label>
                        }
                    }).collect_view()}
                </fieldset>
                <div class="request-row">
                    <div class="field">
                        <label class="field-label" for="req-name">"Name"</label>
                        <input id="req-name" class="field-input" type="text" autocomplete="name" required prop:value=move || name.get() on:input=move |ev| name.set(event_target_value(&ev))/>
                    </div>
                    <div class="field">
                        <label class="field-label" for="req-email">"Email"</label>
                        <input id="req-email" class="field-input" type="email" autocomplete="email" required prop:value=move || email.get() on:input=move |ev| email.set(event_target_value(&ev))/>
                    </div>
                    <div class="field">
                        <label class="field-label" for="req-purpose">"How will you use them?"</label>
                        <select id="req-purpose" class="field-input" on:change=move |ev| purpose.set(event_target_value(&ev))>
                            {PURPOSES.iter().map(|(k, l)| view! { <option value=*k selected=move || purpose.get() == *k>{*l}</option> }).collect_view()}
                        </select>
                    </div>
                </div>
                <div class="field">
                    <label class="field-label" for="req-message">"Message"<span class="field-hint">"Optional"</span></label>
                    <textarea id="req-message" class="field-input" rows="3" prop:value=move || message.get() on:input=move |ev| message.set(event_target_value(&ev))></textarea>
                </div>
                <div class="request-actions">
                    <button type="submit" class="btn btn-primary" disabled=move || status.get() == Status::Sending>
                        {move || match (EVIDENCE_REQUEST_ENDPOINT, EVIDENCE_REQUEST_EMAIL) {
                            (Some(_), _) => "Send request",
                            (None, Some(_)) => "Open in your mail app",
                            (None, None) => "Copy request",
                        }}
                    </button>
                    {EVIDENCE_REQUEST_EMAIL.filter(|_| EVIDENCE_REQUEST_ENDPOINT.is_none()).map(|address| view! {
                        <span class="request-to mono">"to "<a href=format!("mailto:{address}")>{address}</a></span>
                    })}
                    <span class="request-status" class:ok=move || matches!(status.get(), Status::Sent | Status::Copied) class:err=move || matches!(status.get(), Status::Failed(_)) role="status" aria-live="polite">
                        {move || match status.get() {
                            Status::Idle => String::new(),
                            Status::Sending => "Sending…".to_string(),
                            Status::Sent => if EVIDENCE_REQUEST_ENDPOINT.is_some() { "Request sent. The team will reply by email.".to_string() } else { "Your mail app should open with the request filled in. If it did not, use the mail link or the text below.".to_string() },
                            Status::Copied => "Request copied. Send it to the Dilate team using the text below.".to_string(),
                            Status::Failed(e) => e,
                        }}
                        {move || mail_link.get().map(|l| view! { " "<a class="request-mail-link" href=l>"Open the email again"</a> })}
                    </span>
                </div>
                {move || error.get().map(|e| view! { <p class="lab-error" role="alert">{e}</p> })}
                {move || output.get().map(|t| view! { <textarea class="request-output" readonly aria-label="Request text" prop:value=t></textarea> })}
            </form>
        </details>
    }
}
