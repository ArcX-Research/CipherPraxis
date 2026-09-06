//! Content model for Cipher Praxis.
//!
//! Content is authored as one TOML file per entry under `content/<section>/<id>.toml`
//! (see `content/AUTHORING.md`). At build time the web crate loads, validates and lints
//! every entry (feature `authoring`), then bundles them into a single JSON document that is
//! embedded in the WebAssembly binary and parsed into a [`Catalog`] at startup.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Top-level information-architecture sections, in navigation order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Section {
    Ciphers,
    Algebra,
    Cryptanalysis,
    Statistics,
    Search,
    Exact,
    Validation,
    Engineering,
    Labs,
    Glossary,
    References,
}

impl Section {
    pub const ALL: [Section; 11] = [
        Section::Ciphers,
        Section::Algebra,
        Section::Cryptanalysis,
        Section::Statistics,
        Section::Search,
        Section::Exact,
        Section::Validation,
        Section::Engineering,
        Section::Labs,
        Section::Glossary,
        Section::References,
    ];

    /// URL segment and content directory name.
    pub fn slug(self) -> &'static str {
        match self {
            Section::Ciphers => "ciphers",
            Section::Algebra => "algebra",
            Section::Cryptanalysis => "cryptanalysis",
            Section::Statistics => "statistics",
            Section::Search => "search",
            Section::Exact => "exact",
            Section::Validation => "validation",
            Section::Engineering => "engineering",
            Section::Labs => "labs",
            Section::Glossary => "glossary",
            Section::References => "references",
        }
    }

    pub fn from_slug(slug: &str) -> Option<Section> {
        Section::ALL.into_iter().find(|s| s.slug() == slug)
    }

    /// Navigation title.
    pub fn title(self) -> &'static str {
        match self {
            Section::Ciphers => "Cipher Systems",
            Section::Algebra => "Mathematical Foundations",
            Section::Cryptanalysis => "Cryptanalysis",
            Section::Statistics => "Statistical Analysis",
            Section::Search => "Search Methods",
            Section::Exact => "Exact Solvers",
            Section::Validation => "Validation",
            Section::Engineering => "Solver Engineering",
            Section::Labs => "Interactive Labs",
            Section::Glossary => "Glossary",
            Section::References => "References",
        }
    }

    /// Short label for compact navigation.
    pub fn short(self) -> &'static str {
        match self {
            Section::Ciphers => "Cipher Systems",
            Section::Algebra => "Mathematical Foundations",
            Section::Cryptanalysis => "Cryptanalysis",
            Section::Statistics => "Statistical Analysis",
            Section::Search => "Search Methods",
            Section::Exact => "Exact Solvers",
            Section::Validation => "Validation",
            Section::Engineering => "Solver Engineering",
            Section::Labs => "Interactive Labs",
            Section::Glossary => "Glossary",
            Section::References => "References",
        }
    }

    /// Two-digit index used as a mono annotation in the UI.
    pub fn ordinal(self) -> usize {
        Section::ALL.iter().position(|s| *s == self).unwrap_or(0) + 1
    }

    pub fn blurb(self) -> &'static str {
        match self {
            Section::Ciphers => "Learn how each cipher works, how its key is built, what stays unchanged, and how it can be broken.",
            Section::Algebra => "Use modular arithmetic, group theory, and linear algebra to turn cipher rules into equations you can solve.",
            Section::Cryptanalysis => "Methods for finding periods, using known text, comparing ciphertexts, and untangling layered ciphers.",
            Section::Statistics => "Tests that show how strong a result is, how uncertain it is, and whether chance could explain it.",
            Section::Search => "Practical ways to explore key spaces that are too large to check one key at a time.",
            Section::Exact => "Methods that check every allowed answer and can prove when no answer exists within a stated scope.",
            Section::Validation => "Controls, audits, and saved records that show whether a test works and what its result really means.",
            Section::Engineering => "How the tools are built, tested, run, and made repeatable.",
            Section::Labs => "Run the same cipher and analysis code in your browser and see each step.",
            Section::Glossary => "Plain definitions for terms used across the site, with links to fuller explanations.",
            Section::References => "Books, papers, and project records that support the claims on this site.",
        }
    }

    /// Whether entries in this section render as full method pages.
    pub fn is_method_section(self) -> bool {
        !matches!(self, Section::Glossary | Section::References)
    }
}

/// Evidence status of an entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "PROMISING")]
    Promising,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "POWER-LIMITED")]
    PowerLimited,
    #[serde(rename = "INCONCLUSIVE")]
    Inconclusive,
    #[serde(rename = "UNTESTED")]
    Untested,
}

impl Status {
    pub const ALL: [Status; 6] = [
        Status::Verified,
        Status::Promising,
        Status::Closed,
        Status::PowerLimited,
        Status::Inconclusive,
        Status::Untested,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Status::Verified => "VERIFIED",
            Status::Promising => "PROMISING",
            Status::Closed => "CLOSED",
            Status::PowerLimited => "POWER-LIMITED",
            Status::Inconclusive => "INCONCLUSIVE",
            Status::Untested => "UNTESTED",
        }
    }

    /// CSS modifier.
    pub fn slug(self) -> &'static str {
        match self {
            Status::Verified => "verified",
            Status::Promising => "promising",
            Status::Closed => "closed",
            Status::PowerLimited => "power-limited",
            Status::Inconclusive => "inconclusive",
            Status::Untested => "untested",
        }
    }

    pub fn from_label(label: &str) -> Option<Status> {
        Status::ALL.into_iter().find(|s| s.label() == label)
    }

    pub fn description(self) -> &'static str {
        match self {
            Status::Verified => "The stated claim has a cited derivation or recorded validation. Read the status note for the assumptions and checks.",
            Status::Promising => "The early evidence is positive, but some matching checks or audits are still missing.",
            Status::Closed => "A completed search or test gave a negative result within a defined scope. The entry states whether this is an exact exclusion or statistical evidence.",
            Status::PowerLimited => "Recovery or detection was insufficient under the tested conditions and computing budget. A miss gives limited evidence of absence.",
            Status::Inconclusive => "The evidence is mixed, an audit is unresolved, or a run was found to be invalid.",
            Status::Untested => "Explained here, but not yet tested.",
        }
    }
}

/// Fixed vocabulary of page blocks, in canonical display order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockKind {
    Definition,
    Equations,
    Variants,
    Assumptions,
    Attack,
    Pseudocode,
    Complexity,
    FailureModes,
    Controls,
    Example,
    Notes,
    History,
}

impl BlockKind {
    pub const ALL: [BlockKind; 12] = [
        BlockKind::Definition,
        BlockKind::Equations,
        BlockKind::Variants,
        BlockKind::Assumptions,
        BlockKind::Attack,
        BlockKind::Pseudocode,
        BlockKind::Complexity,
        BlockKind::FailureModes,
        BlockKind::Controls,
        BlockKind::Example,
        BlockKind::Notes,
        BlockKind::History,
    ];

    pub fn display_name(self) -> &'static str {
        match self {
            BlockKind::Definition => "Definition",
            BlockKind::Equations => "Math and formulas",
            BlockKind::Variants => "Types and variants",
            BlockKind::Assumptions => "Assumptions & invariants",
            BlockKind::Attack => "How to attack it",
            BlockKind::Pseudocode => "Pseudocode",
            BlockKind::Complexity => "Cost and limits",
            BlockKind::FailureModes => "When it can fail",
            BlockKind::Controls => "Checks and controls",
            BlockKind::Example => "Worked example",
            BlockKind::Notes => "Notes",
            BlockKind::History => "History",
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            BlockKind::Definition => "definition",
            BlockKind::Equations => "equations",
            BlockKind::Variants => "variants",
            BlockKind::Assumptions => "assumptions",
            BlockKind::Attack => "attack",
            BlockKind::Pseudocode => "pseudocode",
            BlockKind::Complexity => "complexity",
            BlockKind::FailureModes => "failure-modes",
            BlockKind::Controls => "controls",
            BlockKind::Example => "example",
            BlockKind::Notes => "notes",
            BlockKind::History => "history",
        }
    }

    /// Two-digit annotation shown next to the block heading.
    pub fn ordinal(self) -> usize {
        BlockKind::ALL.iter().position(|k| *k == self).unwrap_or(0) + 1
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub kind: BlockKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub body: String,
}

impl Block {
    pub fn heading(&self) -> String {
        self.title
            .clone()
            .unwrap_or_else(|| self.kind.display_name().to_string())
    }
}

/// Internal evidence pointer. `path` is relative to the evidence corpus root.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provenance {
    pub path: String,
    #[serde(default = "default_provenance_kind")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, rename = "ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

fn default_provenance_kind() -> String {
    "note".to_string()
}

pub const PROVENANCE_KINDS: &[&str] = &[
    "solver", "library", "audit", "design", "note", "log", "script", "data", "ledger",
];

/// External literature reference.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// One knowledge-base record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub section: Section,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    pub status: Status,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub related: Vec<String>,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// Labs only: key of the interactive component.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lab: Option<String>,
    #[serde(default)]
    pub blocks: Vec<Block>,
    #[serde(default)]
    pub provenance: Vec<Provenance>,
    #[serde(default)]
    pub references: Vec<Reference>,
}

impl Entry {
    pub fn route(&self) -> String {
        format!("/{}/{}", self.section.slug(), self.id)
    }

    pub fn family_or_default(&self) -> &str {
        self.family.as_deref().unwrap_or("General")
    }

    /// Plain-text concatenation of all public prose, used by the search index and the lint.
    pub fn public_text(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.title);
        s.push('\n');
        if let Some(sub) = &self.subtitle {
            s.push_str(sub);
            s.push('\n');
        }
        s.push_str(&self.summary);
        s.push('\n');
        if let Some(n) = &self.status_note {
            s.push_str(n);
            s.push('\n');
        }
        if let Some(f) = &self.family {
            s.push_str(f);
            s.push('\n');
        }
        for t in &self.tags {
            s.push_str(t);
            s.push('\n');
        }
        for b in &self.blocks {
            if let Some(t) = &b.title {
                s.push_str(t);
                s.push('\n');
            }
            s.push_str(&b.body);
            s.push('\n');
        }
        s
    }
}

/// The serialized bundle embedded in the binary.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bundle {
    pub generated: String,
    pub entries: Vec<Entry>,
}

/// In-memory, indexed view of all entries.
#[derive(Debug)]
pub struct Catalog {
    entries: Vec<Entry>,
    by_id: HashMap<String, usize>,
    by_section: BTreeMap<Section, Vec<usize>>,
    backlinks: HashMap<String, Vec<usize>>,
    generated: String,
}

impl Catalog {
    pub fn from_entries(mut entries: Vec<Entry>, generated: String) -> Catalog {
        entries.sort_by(|a, b| {
            a.section
                .cmp(&b.section)
                .then_with(|| a.family_or_default().cmp(b.family_or_default()))
                .then_with(|| a.title.to_lowercase().cmp(&b.title.to_lowercase()))
        });
        let mut by_id = HashMap::new();
        let mut by_section: BTreeMap<Section, Vec<usize>> = BTreeMap::new();
        for (i, e) in entries.iter().enumerate() {
            by_id.insert(e.id.clone(), i);
            by_section.entry(e.section).or_default().push(i);
        }
        let mut backlinks: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, e) in entries.iter().enumerate() {
            for r in &e.related {
                if by_id.contains_key(r) {
                    backlinks.entry(r.clone()).or_default().push(i);
                }
            }
        }
        Catalog {
            entries,
            by_id,
            by_section,
            backlinks,
            generated,
        }
    }

    pub fn from_json(json: &str) -> Result<Catalog, String> {
        let bundle: Bundle = serde_json::from_str(json).map_err(|e| e.to_string())?;
        Ok(Catalog::from_entries(bundle.entries, bundle.generated))
    }

    pub fn generated(&self) -> &str {
        &self.generated
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn get(&self, id: &str) -> Option<&Entry> {
        self.by_id.get(id).map(|&i| &self.entries[i])
    }

    pub fn section(&self, section: Section) -> Vec<&Entry> {
        self.by_section
            .get(&section)
            .map(|v| v.iter().map(|&i| &self.entries[i]).collect())
            .unwrap_or_default()
    }

    pub fn section_count(&self, section: Section) -> usize {
        self.by_section.get(&section).map(|v| v.len()).unwrap_or(0)
    }

    /// Distinct families in a section, in display order.
    pub fn families(&self, section: Section) -> Vec<String> {
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        for e in self.section(section) {
            let f = e.family_or_default().to_string();
            if seen.insert(f.clone()) {
                out.push(f);
            }
        }
        out
    }

    /// Distinct tags in a section with counts, most frequent first.
    pub fn tags(&self, section: Section) -> Vec<(String, usize)> {
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for e in self.section(section) {
            for t in &e.tags {
                *counts.entry(t.clone()).or_default() += 1;
            }
        }
        let mut v: Vec<(String, usize)> = counts.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        v
    }

    /// Resolved outgoing links (missing ids are skipped).
    pub fn related(&self, entry: &Entry) -> Vec<&Entry> {
        entry.related.iter().filter_map(|r| self.get(r)).collect()
    }

    /// Entries that link to `id` but are not linked back from it.
    pub fn backlinks(&self, id: &str) -> Vec<&Entry> {
        let forward: BTreeSet<&str> = self
            .get(id)
            .map(|e| e.related.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default();
        self.backlinks
            .get(id)
            .map(|v| {
                v.iter()
                    .map(|&i| &self.entries[i])
                    .filter(|e| !forward.contains(e.id.as_str()))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn status_counts(&self) -> Vec<(Status, usize)> {
        Status::ALL
            .iter()
            .map(|s| (*s, self.entries.iter().filter(|e| e.status == *s).count()))
            .collect()
    }

    /// All distinct provenance paths with the entries citing them.
    pub fn provenance_map(&self) -> BTreeMap<String, Vec<&Entry>> {
        let mut m: BTreeMap<String, Vec<&Entry>> = BTreeMap::new();
        for e in &self.entries {
            for p in &e.provenance {
                let v = m.entry(p.path.clone()).or_default();
                if !v.iter().any(|x| x.id == e.id) {
                    v.push(e);
                }
            }
        }
        m
    }
}

// ---------------------------------------------------------------------------------------------
// Validation and public-text lint
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Error,
    Warning,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Issue {
    pub level: Level,
    pub entry: String,
    pub message: String,
}

impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lvl = match self.level {
            Level::Error => "error",
            Level::Warning => "warning",
        };
        write!(f, "{lvl}: [{}] {}", self.entry, self.message)
    }
}

/// Tokens that must not appear in public text (case-insensitive, word-bounded).
pub const FORBIDDEN_TOKENS: &[&str] = &[
    "kryptos",
    "ctf",
    "sanborn",
    "langley",
    "cia",
    "pk0",
    "pk1",
    "pk2",
    "pk3",
    "pk4",
    "pk5",
    "pk6",
    "pk7",
    "pk8",
    "pk9",
    "pk10",
    "pk89",
    "pk98",
    "pk8910",
    "leaderboard",
    "submission",
    "poem",
    "paradigm",
];

/// Tokens that are only suspicious (reported as warnings).
pub const SUSPICIOUS_TOKENS: &[&str] = &["k1", "k2", "k3", "k4", "hint", "crib list", "story"];

fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
}

/// Returns the forbidden and suspicious tokens found in `text`.
pub fn lint_public_text(text: &str) -> (Vec<String>, Vec<String>) {
    let mut forbidden = BTreeSet::new();
    let mut suspicious = BTreeSet::new();
    for w in words(text) {
        if FORBIDDEN_TOKENS.contains(&w.as_str()) {
            forbidden.insert(w.clone());
        }
        if SUSPICIOUS_TOKENS.contains(&w.as_str()) {
            suspicious.insert(w);
        }
    }
    let lower = text.to_lowercase();
    for phrase in SUSPICIOUS_TOKENS.iter().filter(|p| p.contains(' ')) {
        if lower.contains(phrase) {
            suspicious.insert(phrase.to_string());
        }
    }
    (
        forbidden.into_iter().collect(),
        suspicious.into_iter().collect(),
    )
}

fn is_slug(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !s.starts_with('-')
        && !s.ends_with('-')
}

/// Structural validation plus the public-text lint. `known_labs` lists the lab component keys
/// the application implements.
pub fn validate(entries: &[Entry], known_labs: &[&str]) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut seen: HashMap<&str, usize> = HashMap::new();
    let ids: BTreeSet<&str> = entries.iter().map(|e| e.id.as_str()).collect();

    for e in entries {
        let push = |issues: &mut Vec<Issue>, level: Level, msg: String| {
            issues.push(Issue {
                level,
                entry: e.id.clone(),
                message: msg,
            })
        };
        if !is_slug(&e.id) {
            push(
                &mut issues,
                Level::Error,
                "id must be a lowercase kebab-case slug".into(),
            );
        }
        *seen.entry(e.id.as_str()).or_default() += 1;
        if e.title.trim().is_empty() {
            push(&mut issues, Level::Error, "title is empty".into());
        }
        if e.summary.trim().len() < 40 {
            push(
                &mut issues,
                Level::Error,
                "summary must be at least 40 characters".into(),
            );
        }
        if e.section.is_method_section() && e.status_note.as_deref().unwrap_or("").trim().is_empty()
        {
            push(
                &mut issues,
                Level::Error,
                "status_note is required for method entries".into(),
            );
        }
        if e.section == Section::Glossary && !e.id.starts_with("g-") {
            push(
                &mut issues,
                Level::Error,
                "glossary ids must start with `g-`".into(),
            );
        }
        if e.section == Section::Labs {
            match &e.lab {
                None => push(
                    &mut issues,
                    Level::Error,
                    "labs entries need a `lab` key".into(),
                ),
                Some(k) if !known_labs.contains(&k.as_str()) => push(
                    &mut issues,
                    Level::Error,
                    format!(
                        "unknown lab component `{k}` (known: {})",
                        known_labs.join(", ")
                    ),
                ),
                _ => {}
            }
        } else if e.lab.is_some() {
            push(
                &mut issues,
                Level::Error,
                "`lab` is only allowed in the labs section".into(),
            );
        }
        if e.section.is_method_section() && e.section != Section::Labs && e.blocks.is_empty() {
            push(
                &mut issues,
                Level::Error,
                "method entries need at least one block".into(),
            );
        }
        // Block order must follow the canonical vocabulary order.
        let mut last = 0usize;
        for b in &e.blocks {
            let ord = b.kind.ordinal();
            if ord < last {
                push(
                    &mut issues,
                    Level::Warning,
                    format!("block `{}` is out of canonical order", b.kind.slug()),
                );
            }
            last = ord.max(last);
            if b.body.trim().is_empty() {
                push(
                    &mut issues,
                    Level::Error,
                    format!("block `{}` has an empty body", b.kind.slug()),
                );
            }
        }
        for p in &e.provenance {
            if !PROVENANCE_KINDS.contains(&p.kind.as_str()) {
                push(
                    &mut issues,
                    Level::Error,
                    format!(
                        "provenance kind `{}` is not one of {}",
                        p.kind,
                        PROVENANCE_KINDS.join(", ")
                    ),
                );
            }
            if p.path.trim().is_empty() || p.path.starts_with('/') || p.path.contains("..") {
                push(
                    &mut issues,
                    Level::Error,
                    format!("provenance path `{}` must be corpus-relative", p.path),
                );
            }
        }
        if e.section.is_method_section() && e.provenance.is_empty() && e.section != Section::Labs {
            push(
                &mut issues,
                Level::Warning,
                "no provenance: claims are not traceable".into(),
            );
        }
        for r in &e.related {
            if r == &e.id {
                push(
                    &mut issues,
                    Level::Warning,
                    "entry lists itself as related".into(),
                );
            } else if !ids.contains(r.as_str()) {
                push(
                    &mut issues,
                    Level::Warning,
                    format!("related id `{r}` does not exist"),
                );
            }
        }
        // Public text lint.
        let (forbidden, suspicious) = lint_public_text(&e.public_text());
        if !forbidden.is_empty() {
            push(
                &mut issues,
                Level::Error,
                format!(
                    "forbidden token(s) in public text: {}",
                    forbidden.join(", ")
                ),
            );
        }
        if !suspicious.is_empty() {
            push(
                &mut issues,
                Level::Warning,
                format!(
                    "suspicious token(s) in public text, please review: {}",
                    suspicious.join(", ")
                ),
            );
        }
        // Internal links must point at existing routes.
        for link in internal_links(&e.public_text()) {
            let mut parts = link.trim_start_matches('/').splitn(2, '/');
            let sec = parts.next().unwrap_or("");
            let id = parts.next();
            match (Section::from_slug(sec), id) {
                (Some(_), None) => {}
                (Some(_), Some(id)) if ids.contains(id) => {}
                _ => push(
                    &mut issues,
                    Level::Warning,
                    format!("internal link `{link}` does not resolve"),
                ),
            }
        }
    }
    for (id, n) in seen {
        if n > 1 {
            issues.push(Issue {
                level: Level::Error,
                entry: id.to_string(),
                message: format!("duplicate id ({n} entries)"),
            });
        }
    }
    issues.sort_by(|a, b| {
        a.entry
            .cmp(&b.entry)
            .then_with(|| a.message.cmp(&b.message))
    });
    issues
}

/// Extracts Markdown link targets that start with `/`.
pub fn internal_links(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i + 2 < bytes.len() {
        if bytes[i] == b']' && bytes[i + 1] == b'(' && bytes[i + 2] == b'/' {
            let start = i + 2;
            let mut j = start;
            while j < bytes.len() && bytes[j] != b')' && !bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if let Ok(s) = std::str::from_utf8(&bytes[start..j]) {
                let s = s.split('#').next().unwrap_or(s);
                out.push(s.to_string());
            }
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Loading from TOML (native only)
// ---------------------------------------------------------------------------------------------

#[cfg(feature = "authoring")]
pub mod load {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Loads every `content/<section>/*.toml` file. Returns entries plus structural issues
    /// found while loading (parse errors, section/directory or id/file-name mismatches).
    pub fn load_dir(root: &Path) -> (Vec<Entry>, Vec<Issue>, Vec<PathBuf>) {
        let mut entries = Vec::new();
        let mut issues = Vec::new();
        let mut files = Vec::new();
        for section in Section::ALL {
            let dir = root.join(section.slug());
            let Ok(rd) = fs::read_dir(&dir) else { continue };
            let mut paths: Vec<PathBuf> = rd
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().map(|x| x == "toml").unwrap_or(false))
                .collect();
            paths.sort();
            for path in paths {
                files.push(path.clone());
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                let text = match fs::read_to_string(&path) {
                    Ok(t) => t,
                    Err(err) => {
                        issues.push(Issue {
                            level: Level::Error,
                            entry: name,
                            message: format!("cannot read {}: {err}", path.display()),
                        });
                        continue;
                    }
                };
                match toml::from_str::<Entry>(&text) {
                    Ok(entry) => {
                        if entry.id != name {
                            issues.push(Issue {
                                level: Level::Error,
                                entry: entry.id.clone(),
                                message: format!("id does not match file name `{name}.toml`"),
                            });
                        }
                        if entry.section != section {
                            issues.push(Issue {
                                level: Level::Error,
                                entry: entry.id.clone(),
                                message: format!(
                                    "section `{}` does not match directory `{}`",
                                    entry.section.slug(),
                                    section.slug()
                                ),
                            });
                        }
                        entries.push(entry);
                    }
                    Err(err) => issues.push(Issue {
                        level: Level::Error,
                        entry: name,
                        message: format!(
                            "TOML parse error in {}: {}",
                            path.display(),
                            err.message()
                        ),
                    }),
                }
            }
        }
        (entries, issues, files)
    }

    /// Deterministic bundle stamp: `SOURCE_DATE_EPOCH` (as an ISO date) when set, otherwise the
    /// latest `updated` date across all entries. Never wall-clock time, so a clean rebuild of the
    /// same content produces byte-identical output.
    pub fn stamp(entries: &[Entry], source_date_epoch: Option<&str>) -> String {
        if let Some(epoch) = source_date_epoch.and_then(|s| s.trim().parse::<i64>().ok()) {
            let (y, m, d) = civil_from_days(epoch.div_euclid(86_400));
            return format!("{y:04}-{m:02}-{d:02} (SOURCE_DATE_EPOCH)");
        }
        entries
            .iter()
            .filter_map(|e| e.updated.as_deref())
            .max()
            .map(|d| format!("content {d}"))
            .unwrap_or_else(|| "content undated".to_string())
    }

    fn civil_from_days(z: i64) -> (i64, u32, u32) {
        // Howard Hinnant's days-to-civil algorithm.
        let z = z + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        (if m <= 2 { y + 1 } else { y }, m, d)
    }

    pub fn bundle_json(entries: &[Entry], generated: &str) -> String {
        let bundle = Bundle {
            generated: generated.to_string(),
            entries: entries.to_vec(),
        };
        serde_json::to_string(&bundle).expect("bundle serializes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, section: Section) -> Entry {
        Entry {
            id: id.into(),
            section,
            title: "Title".into(),
            subtitle: None,
            status: Status::Verified,
            status_note: Some("Because tests.".into()),
            family: Some("Family".into()),
            tags: vec![],
            related: vec![],
            summary: "A summary that is comfortably longer than forty characters in total.".into(),
            updated: None,
            lab: None,
            blocks: vec![Block {
                kind: BlockKind::Definition,
                title: None,
                body: "Body".into(),
            }],
            provenance: vec![Provenance {
                path: "withmath/x.py".into(),
                kind: "solver".into(),
                note: None,
                reference: None,
            }],
            references: vec![],
        }
    }

    #[test]
    fn lint_catches_forbidden_tokens() {
        let (f, s) = lint_public_text("The Kryptos sculpture and a PK8 text; a hint.");
        assert_eq!(f, vec!["kryptos".to_string(), "pk8".to_string()]);
        assert_eq!(s, vec!["hint".to_string()]);
    }

    #[test]
    fn lint_ignores_math_subscripts() {
        let (f, s) = lint_public_text("$k_1 + k_2$ and cosets");
        assert!(f.is_empty());
        assert!(s.is_empty());
    }

    #[test]
    fn validate_flags_duplicates_and_dangling_links() {
        let mut a = entry("alpha", Section::Ciphers);
        a.related = vec!["missing".into()];
        let b = entry("alpha", Section::Algebra);
        let issues = validate(&[a, b], &[]);
        assert!(issues.iter().any(|i| i.message.contains("duplicate id")));
        assert!(issues.iter().any(|i| i.message.contains("does not exist")));
    }

    #[test]
    fn validate_requires_lab_key() {
        let mut l = entry("vigenere-lab", Section::Labs);
        l.lab = Some("nope".into());
        let issues = validate(&[l.clone()], &["vigenere"]);
        assert!(issues
            .iter()
            .any(|i| i.level == Level::Error && i.message.contains("unknown lab")));
        l.lab = Some("vigenere".into());
        let issues = validate(&[l], &["vigenere"]);
        assert!(issues.iter().all(|i| i.level != Level::Error));
    }

    #[test]
    fn catalog_indexes_sections_and_backlinks() {
        let mut a = entry("a", Section::Ciphers);
        a.related = vec!["b".into()];
        let b = entry("b", Section::Algebra);
        let c = Catalog::from_entries(vec![a, b], "now".into());
        assert_eq!(c.section(Section::Ciphers).len(), 1);
        assert_eq!(c.backlinks("b").len(), 1);
        assert_eq!(c.get("a").unwrap().route(), "/ciphers/a");
    }

    #[test]
    fn internal_link_extraction() {
        let links =
            internal_links("see [x](/ciphers/beaufort) and [y](/labs) and [z](https://e.com)");
        assert_eq!(
            links,
            vec!["/ciphers/beaufort".to_string(), "/labs".to_string()]
        );
    }

    #[cfg(feature = "authoring")]
    #[test]
    fn bundle_is_deterministic() {
        let mut a = entry("a", Section::Ciphers);
        a.updated = Some("2026-09-01".into());
        let mut b = entry("b", Section::Algebra);
        b.updated = Some("2026-09-04".into());
        let entries = vec![a, b];
        assert_eq!(load::stamp(&entries, None), "content 2026-09-04");
        assert_eq!(
            load::stamp(&entries, Some("1700000000")),
            "2023-11-14 (SOURCE_DATE_EPOCH)"
        );
        let s1 = load::bundle_json(&entries, &load::stamp(&entries, None));
        let s2 = load::bundle_json(&entries.clone(), &load::stamp(&entries, None));
        assert_eq!(s1, s2);
        assert!(!s1.contains("T00:"));
    }
}
