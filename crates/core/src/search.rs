//! A small in-memory full-text index built at startup from the catalog.
//!
//! Field-weighted token scoring with prefix matching; good enough for a few hundred entries
//! and instant in WebAssembly.

use crate::content::{Catalog, Section, Status};
use crate::render::markdown_to_text;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug)]
pub struct Doc {
    pub id: String,
    pub section: Section,
    pub status: Status,
    pub title: String,
    pub subtitle: Option<String>,
    pub family: String,
    pub summary: String,
    body: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hit {
    pub id: String,
    pub score: f32,
    pub snippet: String,
}

#[derive(Debug, Default)]
pub struct SearchIndex {
    docs: Vec<Doc>,
    /// token → (doc index, accumulated weight)
    postings: BTreeMap<String, Vec<(usize, f32)>>,
}

/// Folds common diacritics so "vigenère" and "vigenere" match.
pub fn fold(c: char) -> char {
    match c {
        'à' | 'á' | 'â' | 'ä' | 'ã' | 'å' => 'a',
        'è' | 'é' | 'ê' | 'ë' => 'e',
        'ì' | 'í' | 'î' | 'ï' => 'i',
        'ò' | 'ó' | 'ô' | 'ö' | 'õ' => 'o',
        'ù' | 'ú' | 'û' | 'ü' => 'u',
        'ç' => 'c',
        'ñ' => 'n',
        _ => c,
    }
}

pub fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .chars()
        .map(fold)
        .collect::<String>()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 1)
        .map(|w| w.to_string())
        .collect()
}

impl SearchIndex {
    pub fn build(catalog: &Catalog) -> SearchIndex {
        let mut index = SearchIndex::default();
        for e in catalog.entries() {
            let body: String = e
                .blocks
                .iter()
                .map(|b| markdown_to_text(&b.body))
                .collect::<Vec<_>>()
                .join(" ");
            let doc_idx = index.docs.len();
            let mut weights: HashMap<String, f32> = HashMap::new();
            let mut add = |text: &str, w: f32| {
                for t in tokenize(text) {
                    *weights.entry(t).or_default() += w;
                }
            };
            add(&e.title, 10.0);
            add(&e.id.replace('-', " "), 6.0);
            for t in &e.tags {
                add(t, 4.0);
            }
            add(e.family_or_default(), 3.0);
            if let Some(s) = &e.subtitle {
                add(s, 3.0);
            }
            add(&e.summary, 2.0);
            add(&body, 0.5);
            for (tok, w) in weights {
                index.postings.entry(tok).or_default().push((doc_idx, w));
            }
            index.docs.push(Doc {
                id: e.id.clone(),
                section: e.section,
                status: e.status,
                title: e.title.clone(),
                subtitle: e.subtitle.clone(),
                family: e.family_or_default().to_string(),
                summary: markdown_to_text(&e.summary),
                body,
            });
        }
        index
    }

    pub fn len(&self) -> usize {
        self.docs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.docs.is_empty()
    }

    pub fn doc(&self, id: &str) -> Option<&Doc> {
        self.docs.iter().find(|d| d.id == id)
    }

    /// Returns ranked hits. Each query token must match (as a prefix) somewhere in the document.
    pub fn query(&self, q: &str, limit: usize) -> Vec<Hit> {
        let terms = tokenize(q);
        if terms.is_empty() {
            return Vec::new();
        }
        let mut scores: HashMap<usize, (f32, usize)> = HashMap::new();
        for term in &terms {
            let mut matched: HashMap<usize, f32> = HashMap::new();
            // Exact and prefix matches over the sorted token space.
            for (tok, posts) in self.postings.range(term.clone()..) {
                if !tok.starts_with(term.as_str()) {
                    break;
                }
                let factor = if tok == term {
                    1.0
                } else {
                    0.6 * (term.len() as f32 / tok.len() as f32).max(0.3)
                };
                for (doc, w) in posts {
                    let s = matched.entry(*doc).or_default();
                    *s = s.max(w * factor);
                }
            }
            for (doc, w) in matched {
                let e = scores.entry(doc).or_insert((0.0, 0));
                e.0 += w;
                e.1 += 1;
            }
        }
        let lower_q = q.trim().to_lowercase();
        let mut hits: Vec<(usize, f32)> = scores
            .into_iter()
            .filter(|(_, (_, n))| *n == terms.len())
            .map(|(doc, (s, _))| {
                let d = &self.docs[doc];
                let bonus = if d.title.to_lowercase().contains(&lower_q) {
                    8.0
                } else {
                    0.0
                };
                (doc, s + bonus)
            })
            .collect();
        hits.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| self.docs[a.0].title.cmp(&self.docs[b.0].title))
        });
        hits.truncate(limit);
        hits.into_iter()
            .map(|(doc, score)| Hit {
                id: self.docs[doc].id.clone(),
                score,
                snippet: self.snippet(doc, &terms),
            })
            .collect()
    }

    fn snippet(&self, doc: usize, terms: &[String]) -> String {
        let d = &self.docs[doc];
        let hay_summary = d.summary.to_lowercase();
        let hay_body = d.body.to_lowercase();
        for (text, hay) in [(&d.summary, &hay_summary), (&d.body, &hay_body)] {
            for term in terms {
                if let Some(pos) = hay.find(term.as_str()) {
                    return window(text, pos, 150);
                }
            }
        }
        window(&d.summary, 0, 150)
    }
}

fn window(text: &str, pos: usize, width: usize) -> String {
    let start = pos.saturating_sub(width / 3);
    let mut s = start;
    while s > 0 && !text.is_char_boundary(s) {
        s -= 1;
    }
    let mut e = (s + width).min(text.len());
    while e < text.len() && !text.is_char_boundary(e) {
        e += 1;
    }
    let mut out = String::new();
    if s > 0 {
        out.push('…');
    }
    out.push_str(text[s..e].trim());
    if e < text.len() {
        out.push('…');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{Block, BlockKind, Entry};

    fn entry(id: &str, title: &str, summary: &str, tags: &[&str]) -> Entry {
        Entry {
            id: id.into(),
            section: Section::Ciphers,
            title: title.into(),
            subtitle: None,
            status: Status::Verified,
            status_note: None,
            family: Some("Polyalphabetic substitution".into()),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            related: vec![],
            summary: summary.into(),
            updated: None,
            lab: None,
            blocks: vec![Block {
                kind: BlockKind::Definition,
                title: None,
                body: "The key repeats with period $m$.".into(),
            }],
            provenance: vec![],
            references: vec![],
        }
    }

    #[test]
    fn title_matches_rank_first() {
        let c = Catalog::from_entries(
            vec![
                entry(
                    "vigenere",
                    "Vigenère cipher",
                    "Periodic additive substitution.",
                    &["periodic"],
                ),
                entry(
                    "beaufort",
                    "Beaufort cipher",
                    "Reciprocal variant of the Vigenère cipher.",
                    &["periodic"],
                ),
            ],
            "t".into(),
        );
        let idx = SearchIndex::build(&c);
        let hits = idx.query("vigenere", 10);
        assert_eq!(hits[0].id, "vigenere");
        assert_eq!(hits.len(), 2);
        let hits = idx.query("vig", 10);
        assert_eq!(hits[0].id, "vigenere");
    }

    #[test]
    fn all_terms_must_match() {
        let c = Catalog::from_entries(
            vec![
                entry("a", "Alpha", "Something about periods.", &[]),
                entry("b", "Beta", "Nothing here.", &[]),
            ],
            "t".into(),
        );
        let idx = SearchIndex::build(&c);
        assert_eq!(idx.query("alpha period", 10).len(), 1);
        assert!(idx.query("alpha nothing", 10).is_empty());
        assert!(idx.query("", 10).is_empty());
    }

    #[test]
    fn snippet_is_windowed() {
        let long = "x".repeat(400) + " needle " + &"y".repeat(400);
        let c = Catalog::from_entries(vec![entry("a", "Alpha", &long, &[])], "t".into());
        let idx = SearchIndex::build(&c);
        let hits = idx.query("needle", 1);
        assert!(hits[0].snippet.contains("needle"));
        assert!(hits[0].snippet.len() < 200);
    }
}
