//! Small DOM helpers.
use leptos::prelude::*;

pub fn set_title(title: &str) {
    document().set_title(&format!("{title} · Cipher Praxis"));
}

/// `0..n` letters of a text broken into groups of five for display.
pub fn group5(s: &str) -> String {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}
