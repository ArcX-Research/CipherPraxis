//! Loads, validates and lints `content/`, then bundles it into `$OUT_DIR/content.json` which
//! the application embeds with `include_str!`. Content errors fail the build with a readable
//! list; warnings are surfaced as cargo warnings. The bundle stamp is deterministic (latest
//! content `updated` date, or `SOURCE_DATE_EPOCH`), so clean rebuilds are byte-identical.

use praxis_core::content::{load, validate, Level, Section};
use std::{env, fs, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let content = manifest.join("../../content");
    let content = content.canonicalize().unwrap_or(content);
    println!("cargo:rerun-if-changed={}", content.display());
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    for s in Section::ALL {
        println!(
            "cargo:rerun-if-changed={}",
            content.join(s.slug()).display()
        );
    }
    let (entries, mut issues, files) = load::load_dir(&content);
    for f in &files {
        println!("cargo:rerun-if-changed={}", f.display());
    }
    issues.extend(validate(&entries, praxis_core::KNOWN_LABS));
    let mut errors = Vec::new();
    for i in &issues {
        match i.level {
            Level::Error => errors.push(i.to_string()),
            Level::Warning => println!("cargo:warning=content {i}"),
        }
    }
    if !errors.is_empty() {
        panic!(
            "\n\n{} content error(s) in {}:\n  {}\n\n",
            errors.len(),
            content.display(),
            errors.join("\n  ")
        );
    }
    let stamp = load::stamp(&entries, env::var("SOURCE_DATE_EPOCH").ok().as_deref());
    let json = load::bundle_json(&entries, &stamp);
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("content.json");
    fs::write(&out, json).expect("write content bundle");
}
