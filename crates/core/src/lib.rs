//! Cipher Praxis core: content model, validation, search, Markdown+MathML rendering,
//! and the cipher / statistics / number-theory code behind the interactive labs.
pub mod content;
pub mod crypto;
pub mod render;
pub mod search;

/// Lab component keys implemented by the web application. The content validator rejects a
/// `labs` entry whose `lab` key is not listed here.
pub const KNOWN_LABS: &[&str] = &[
    "vigenere",
    "quagmire",
    "coincidence",
    "affine",
    "columnar",
    "hill",
    "bifid",
    "crt",
    "dihedral",
    "null",
];
