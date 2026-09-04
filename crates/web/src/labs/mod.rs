//! Interactive labs. Each lab is a Leptos component around `praxis_core::crypto`.
//! The keys here must match `praxis_core::KNOWN_LABS` (checked by a test).
use leptos::prelude::*;

mod affine;
mod bifid;
mod coincidence;
mod columnar;
mod crt;
mod dihedral;
mod hill;
mod null;
mod quagmire;
mod vigenere;
pub mod widgets;

#[cfg(test)]
const LAB_KEYS: &[&str] = &[
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

pub fn render_lab(key: &str) -> AnyView {
    match key {
        "vigenere" => view! { <vigenere::VigenereLab/> }.into_any(),
        "quagmire" => view! { <quagmire::QuagmireLab/> }.into_any(),
        "coincidence" => view! { <coincidence::CoincidenceLab/> }.into_any(),
        "affine" => view! { <affine::AffineLab/> }.into_any(),
        "columnar" => view! { <columnar::ColumnarLab/> }.into_any(),
        "hill" => view! { <hill::HillLab/> }.into_any(),
        "bifid" => view! { <bifid::BifidLab/> }.into_any(),
        "crt" => view! { <crt::CrtLab/> }.into_any(),
        "dihedral" => view! { <dihedral::DihedralLab/> }.into_any(),
        "null" => view! { <null::NullLab/> }.into_any(),
        other => {
            view! { <p class="lab-missing">{format!("Unknown lab “{other}”.")}</p> }.into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lab_keys_match_core_registry() {
        assert_eq!(super::LAB_KEYS, praxis_core::KNOWN_LABS);
    }
}
