//! `praxis-check`: validates and lints the content directory, prints a summary.
//!
//! Usage: cargo run -p praxis-core --features authoring --bin praxis-check -- [content-dir] [--strict]

use praxis_core::content::{load, validate, Entry, Level, Section};
use praxis_core::render::markdown_to_html;
use std::path::PathBuf;
use std::process::exit;

/// Lab component keys implemented by the web application (kept in sync with
/// `crates/web/src/labs/mod.rs`, which has a test asserting equality).
pub const KNOWN_LABS: &[&str] = praxis_core::KNOWN_LABS;

/// Visible text of an HTML fragment with MathML (and its LaTeX annotations) removed.
fn visible_text(html: &str) -> String {
    // Drop MathML (whose annotations carry LaTeX) and code, where dollars are literal by design.
    let mut out = String::new();
    let mut rest = html;
    loop {
        let next = ["<math", "<pre", "<code"]
            .iter()
            .filter_map(|tag| rest.find(tag).map(|i| (i, *tag)))
            .min_by_key(|(i, _)| *i);
        let Some((i, tag)) = next else { break };
        out.push_str(&rest[..i]);
        let close = match tag {
            "<math" => "</math>",
            "<pre" => "</pre>",
            _ => "</code>",
        };
        match rest[i..].find(close) {
            Some(j) => rest = &rest[i + j + close.len()..],
            None => {
                rest = "";
            }
        }
    }
    out.push_str(rest);
    let mut text = String::new();
    let mut in_tag = false;
    for c in out.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(c),
            _ => {}
        }
    }
    text
}

/// Reports every `$` that survives Markdown+math rendering as visible text, which means the
/// span around it was not accepted as math (for example a space before the closing dollar).
fn lint_math(entries: &[Entry]) -> usize {
    let mut count = 0;
    for e in entries {
        let mut fields: Vec<(String, &str)> = vec![("summary".into(), e.summary.as_str())];
        if let Some(n) = &e.status_note {
            fields.push(("status_note".into(), n.as_str()));
        }
        for b in &e.blocks {
            fields.push((format!("block {}", b.kind.slug()), b.body.as_str()));
        }
        for (name, body) in fields {
            let text = visible_text(&markdown_to_html(body));
            for (pos, _) in text.match_indices('$') {
                count += 1;
                let start = text[..pos]
                    .char_indices()
                    .rev()
                    .nth(30)
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                let end = text[pos..]
                    .char_indices()
                    .nth(40)
                    .map(|(i, _)| pos + i)
                    .unwrap_or(text.len());
                println!(
                    "unhandled $: [{}/{}] {}: …{}…",
                    e.section.slug(),
                    e.id,
                    name,
                    text[start..end].replace('\n', " ")
                );
            }
        }
    }
    count
}

/// Reports list markers that survive rendering as plain text instead of becoming list items:
/// bullet characters, or `- ` / `1. ` at the start of a paragraph or line.
fn lint_lists(entries: &[Entry]) -> usize {
    let mut count = 0;
    for e in entries {
        for b in &e.blocks {
            let html = markdown_to_html(&b.body);
            let text = visible_text(&html);
            for line in text.lines() {
                let t = line.trim_start();
                let bad = t.starts_with("• ")
                    || t.starts_with("- ")
                    || t.starts_with("* ")
                    || t.starts_with("+ ")
                    || t.starts_with("– ")
                    || (t.len() > 3
                        && t.as_bytes()[0].is_ascii_digit()
                        && (t.starts_with(|c: char| c.is_ascii_digit())
                            && t[1..]
                                .trim_start_matches(|c: char| c.is_ascii_digit())
                                .starts_with(". ")));
                if bad {
                    count += 1;
                    println!(
                        "unconverted list marker: [{}/{}] block {}: {}",
                        e.section.slug(),
                        e.id,
                        b.kind.slug(),
                        t.chars().take(80).collect::<String>()
                    );
                }
            }
            // Indented lines inside a fenced-code-free block that became a code block by accident.
            if html.contains("<pre><code>") && !b.body.contains("```") {
                count += 1;
                println!(
                    "indented text rendered as code: [{}/{}] block {}",
                    e.section.slug(),
                    e.id,
                    b.kind.slug()
                );
            }
        }
    }
    count
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let strict = args.iter().any(|a| a == "--strict");
    let math = args.iter().any(|a| a == "--math");
    let lists = args.iter().any(|a| a == "--lists");
    let dir = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content"));
    let (entries, mut issues, files) = load::load_dir(&dir);
    if math {
        let n = lint_math(&entries);
        println!("--\nunhandled dollar signs: {n}");
        exit(if n > 0 { 1 } else { 0 });
    }
    if lists {
        let n = lint_lists(&entries);
        println!("--\nbroken list markers: {n}");
        exit(if n > 0 { 1 } else { 0 });
    }
    issues.extend(validate(&entries, KNOWN_LABS));
    let errors = issues.iter().filter(|i| i.level == Level::Error).count();
    let warnings = issues.len() - errors;
    for i in &issues {
        println!("{i}");
    }
    println!("--");
    println!("content dir: {}", dir.display());
    println!(
        "files: {}  entries: {}  errors: {}  warnings: {}",
        files.len(),
        entries.len(),
        errors,
        warnings
    );
    for s in Section::ALL {
        let n = entries.iter().filter(|e| e.section == s).count();
        if n > 0 {
            println!("  {:<14} {:>3}", s.slug(), n);
        }
    }
    if errors > 0 || (strict && warnings > 0) {
        exit(1);
    }
}
