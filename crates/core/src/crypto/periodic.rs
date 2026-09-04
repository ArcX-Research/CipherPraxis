//! Periodic polyalphabetic substitution: the Vigenère family (Vigenère, Beaufort, variant
//! Beaufort, Porta) and the keyed-alphabet Quagmire I–IV family.

use super::alphabet::{normalize, Alphabet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    Vigenere,
    Beaufort,
    VariantBeaufort,
    Porta,
}

impl Variant {
    pub const ALL: [Variant; 4] = [
        Variant::Vigenere,
        Variant::Beaufort,
        Variant::VariantBeaufort,
        Variant::Porta,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Variant::Vigenere => "Vigenère",
            Variant::Beaufort => "Beaufort",
            Variant::VariantBeaufort => "Variant Beaufort",
            Variant::Porta => "Porta",
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            Variant::Vigenere => "vigenere",
            Variant::Beaufort => "beaufort",
            Variant::VariantBeaufort => "variant-beaufort",
            Variant::Porta => "porta",
        }
    }

    pub fn from_slug(s: &str) -> Option<Variant> {
        Variant::ALL.into_iter().find(|v| v.slug() == s)
    }

    /// Encryption rule as a display formula.
    pub fn formula(self) -> &'static str {
        match self {
            Variant::Vigenere => "c = p + k (mod 26)",
            Variant::Beaufort => "c = k − p (mod 26)",
            Variant::VariantBeaufort => "c = p − k (mod 26)",
            Variant::Porta => "13 reciprocal half-alphabets selected by the key letter pair",
        }
    }
}

fn porta(p: u8, k: u8) -> u8 {
    let r = (k / 2) as i16; // key pair AB → row 0, CD → row 1, …
    let p = p as i16;
    if p < 13 {
        (13 + (p + r).rem_euclid(13)) as u8
    } else {
        ((p - 13 - r).rem_euclid(13)) as u8
    }
}

fn encipher_letter(p: u8, k: u8, v: Variant) -> u8 {
    match v {
        Variant::Vigenere => (p + k) % 26,
        Variant::Beaufort => (26 + k - p) % 26,
        Variant::VariantBeaufort => (26 + p - k) % 26,
        Variant::Porta => porta(p, k),
    }
}

fn decipher_letter(c: u8, k: u8, v: Variant) -> u8 {
    match v {
        Variant::Vigenere => (26 + c - k) % 26,
        Variant::Beaufort => (26 + k - c) % 26,
        Variant::VariantBeaufort => (c + k) % 26,
        Variant::Porta => porta(c, k),
    }
}

fn key_indices(key: &str) -> Result<Vec<u8>, String> {
    let k = normalize(key);
    if k.is_empty() {
        return Err("Enter a key with at least one letter.".into());
    }
    Ok(k.bytes().map(|b| b - b'A').collect())
}

pub fn encrypt(plain: &str, key: &str, v: Variant) -> Result<String, String> {
    let key = key_indices(key)?;
    Ok(normalize(plain)
        .bytes()
        .enumerate()
        .map(|(i, b)| (b'A' + encipher_letter(b - b'A', key[i % key.len()], v)) as char)
        .collect())
}

pub fn decrypt(cipher: &str, key: &str, v: Variant) -> Result<String, String> {
    let key = key_indices(key)?;
    Ok(normalize(cipher)
        .bytes()
        .enumerate()
        .map(|(i, b)| (b'A' + decipher_letter(b - b'A', key[i % key.len()], v)) as char)
        .collect())
}

/// Splits a text into its `m` cosets (positions congruent modulo `m`).
pub fn cosets(text: &str, m: usize) -> Vec<String> {
    let t = normalize(text);
    let m = m.max(1);
    let mut out = vec![String::new(); m];
    for (i, c) in t.chars().enumerate() {
        out[i % m].push(c);
    }
    out
}

/// Quagmire family. Both alphabets straight gives plain Vigenère; the ACA conventions are
/// Quagmire I (keyed plaintext alphabet), II (keyed ciphertext alphabet), III (the same keyed
/// alphabet on both sides) and IV (two different keyed alphabets). The key letters are written
/// under the `indicator` letter of the plaintext alphabet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Quagmire {
    pub plain_alphabet: Alphabet,
    pub cipher_alphabet: Alphabet,
    pub key: Vec<u8>,
    pub indicator: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuagmireKind {
    I,
    II,
    III,
    IV,
}

impl QuagmireKind {
    pub const ALL: [QuagmireKind; 4] = [
        QuagmireKind::I,
        QuagmireKind::II,
        QuagmireKind::III,
        QuagmireKind::IV,
    ];

    pub fn label(self) -> &'static str {
        match self {
            QuagmireKind::I => "Quagmire I",
            QuagmireKind::II => "Quagmire II",
            QuagmireKind::III => "Quagmire III",
            QuagmireKind::IV => "Quagmire IV",
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            QuagmireKind::I => "i",
            QuagmireKind::II => "ii",
            QuagmireKind::III => "iii",
            QuagmireKind::IV => "iv",
        }
    }

    pub fn from_slug(s: &str) -> Option<QuagmireKind> {
        QuagmireKind::ALL.into_iter().find(|k| k.slug() == s)
    }

    pub fn description(self) -> &'static str {
        match self {
            QuagmireKind::I => "keyed plaintext alphabet, straight ciphertext alphabet",
            QuagmireKind::II => "straight plaintext alphabet, keyed ciphertext alphabet",
            QuagmireKind::III => "one keyed alphabet on both sides",
            QuagmireKind::IV => "two different keyed alphabets",
        }
    }
}

impl Quagmire {
    /// Builds the four ACA Quagmire types from keyword(s). `plain_kw` and `cipher_kw` are used
    /// as the kind requires; the indicator defaults to `A` when not a letter.
    pub fn new(
        kind: QuagmireKind,
        plain_kw: &str,
        cipher_kw: &str,
        key: &str,
        indicator: char,
    ) -> Result<Quagmire, String> {
        let pa = Alphabet::keyword(plain_kw);
        let ca = Alphabet::keyword(cipher_kw);
        let (plain_alphabet, cipher_alphabet) = match kind {
            QuagmireKind::I => (pa, Alphabet::straight()),
            QuagmireKind::II => (Alphabet::straight(), ca),
            QuagmireKind::III => (pa.clone(), pa),
            QuagmireKind::IV => (pa, ca),
        };
        let key = key_indices(key)?.iter().map(|k| k + b'A').collect();
        let indicator = if indicator.is_ascii_alphabetic() {
            indicator.to_ascii_uppercase() as u8
        } else {
            b'A'
        };
        Ok(Quagmire {
            plain_alphabet,
            cipher_alphabet,
            key,
            indicator,
        })
    }

    /// Per-coset shift of the ciphertext alphabet relative to the plaintext alphabet.
    pub fn shifts(&self) -> Vec<usize> {
        let ind = self.plain_alphabet.position(self.indicator);
        self.key
            .iter()
            .map(|&k| (26 + self.cipher_alphabet.position(k) - ind) % 26)
            .collect()
    }

    pub fn encrypt(&self, plain: &str) -> String {
        let shifts = self.shifts();
        normalize(plain)
            .bytes()
            .enumerate()
            .map(|(i, p)| {
                let s = shifts[i % shifts.len()];
                self.cipher_alphabet
                    .letter(self.plain_alphabet.position(p) + s) as char
            })
            .collect()
    }

    pub fn decrypt(&self, cipher: &str) -> String {
        let shifts = self.shifts();
        normalize(cipher)
            .bytes()
            .enumerate()
            .map(|(i, c)| {
                let s = shifts[i % shifts.len()];
                self.plain_alphabet
                    .letter(26 + self.cipher_alphabet.position(c) - s) as char
            })
            .collect()
    }

    /// The tableau row used for coset `j`: the ciphertext alphabet rotated by that coset's shift.
    pub fn row(&self, j: usize) -> String {
        let shifts = self.shifts();
        let s = shifts[j % shifts.len()];
        (0..26)
            .map(|i| self.cipher_alphabet.letter(i + s) as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vigenere_known_vector() {
        assert_eq!(
            encrypt("ATTACK AT DAWN", "LEMON", Variant::Vigenere).unwrap(),
            "LXFOPVEFRNHR"
        );
        assert_eq!(
            decrypt("LXFOPVEFRNHR", "LEMON", Variant::Vigenere).unwrap(),
            "ATTACKATDAWN"
        );
    }

    #[test]
    fn beaufort_is_reciprocal() {
        let c = encrypt("ATTACKATDAWN", "LEMON", Variant::Beaufort).unwrap();
        assert_eq!(c, "LLTOLBETLNPR");
        assert_eq!(
            encrypt(&c, "LEMON", Variant::Beaufort).unwrap(),
            "ATTACKATDAWN"
        );
        assert_eq!(
            decrypt(&c, "LEMON", Variant::Beaufort).unwrap(),
            "ATTACKATDAWN"
        );
    }

    #[test]
    fn variant_beaufort_inverts_vigenere() {
        let c = encrypt("HELLOWORLD", "KEY", Variant::VariantBeaufort).unwrap();
        assert_eq!(
            decrypt(&c, "KEY", Variant::VariantBeaufort).unwrap(),
            "HELLOWORLD"
        );
        assert_eq!(
            encrypt("HELLOWORLD", "KEY", Variant::Vigenere)
                .map(|v| decrypt(&v, "KEY", Variant::Vigenere).unwrap())
                .unwrap(),
            "HELLOWORLD"
        );
    }

    #[test]
    fn porta_rows_and_reciprocity() {
        assert_eq!(encrypt("AT", "A", Variant::Porta).unwrap(), "NG");
        assert_eq!(encrypt("AT", "C", Variant::Porta).unwrap(), "OF");
        let c = encrypt("DEFENDTHEEASTWALL", "FORTIFICATION", Variant::Porta).unwrap();
        assert_eq!(
            encrypt(&c, "FORTIFICATION", Variant::Porta).unwrap(),
            "DEFENDTHEEASTWALL"
        );
    }

    #[test]
    fn cosets_split_positions() {
        assert_eq!(cosets("ABCDEFG", 3), vec!["ADG", "BE", "CF"]);
    }

    #[test]
    fn quagmire_reduces_to_vigenere_and_roundtrips() {
        let q = Quagmire::new(QuagmireKind::I, "", "", "LEMON", 'A').unwrap();
        assert_eq!(q.encrypt("ATTACKATDAWN"), "LXFOPVEFRNHR");
        for kind in QuagmireKind::ALL {
            let q = Quagmire::new(kind, "SPRINGFEVER", "FLOWER", "PARTY", 'K').unwrap();
            let c = q.encrypt("THEQUICKBROWNFOXJUMPS");
            assert_eq!(q.decrypt(&c), "THEQUICKBROWNFOXJUMPS", "{kind:?}");
            assert_eq!(q.row(0).len(), 26);
        }
    }

    #[test]
    fn quagmire_iii_indicator_alignment() {
        // With the same alphabet on both sides and key letter equal to the indicator, the
        // shift is zero and the cipher is the identity.
        let q = Quagmire::new(QuagmireKind::III, "KEYWORD", "", "K", 'K').unwrap();
        assert_eq!(q.shifts(), vec![0]);
        assert_eq!(q.encrypt("HELLO"), "HELLO");
    }
}
