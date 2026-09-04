//! `praxis-check`: validates and lints the content directory, prints a summary.
//!
//! Usage: cargo run -p praxis-core --features authoring --bin praxis-check -- [content-dir] [--strict]

use praxis_core::content::{load, validate, Level, Section};
use std::path::PathBuf;
use std::process::exit;

/// Lab component keys implemented by the web application (kept in sync with
/// `crates/web/src/labs/mod.rs`, which has a test asserting equality).
pub const KNOWN_LABS: &[&str] = praxis_core::KNOWN_LABS;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let strict = args.iter().any(|a| a == "--strict");
    let dir = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content"));
    let (entries, mut issues, files) = load::load_dir(&dir);
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
