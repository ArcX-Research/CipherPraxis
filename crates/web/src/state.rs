//! Application-wide state: the content catalog and the search index, provided through context.
use leptos::prelude::*;
use praxis_core::content::Catalog;
use praxis_core::search::SearchIndex;
use std::sync::Arc;

/// The validated content bundle produced by `build.rs`.
pub const CONTENT_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/content.json"));

/// Name of the evidence corpus root that provenance paths are relative to.
pub const CORPUS_ROOT: &str = "poemanalysis";

#[derive(Clone)]
pub struct AppState {
    pub catalog: Arc<Catalog>,
    pub index: Arc<SearchIndex>,
}

impl AppState {
    pub fn load() -> AppState {
        let catalog =
            Catalog::from_json(CONTENT_JSON).expect("embedded content bundle is valid JSON");
        let index = SearchIndex::build(&catalog);
        AppState {
            catalog: Arc::new(catalog),
            index: Arc::new(index),
        }
    }
}

pub fn use_state() -> AppState {
    use_context::<AppState>().expect("AppState provided at the root")
}

/// Open/closed state of the search palette.
#[derive(Clone, Copy)]
pub struct Palette(pub RwSignal<bool>);

pub fn use_palette() -> RwSignal<bool> {
    use_context::<Palette>()
        .expect("Palette provided at the root")
        .0
}
