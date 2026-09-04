//! Alphabets over A–Z: the straight alphabet and keyword-mixed permutations.

pub const STRAIGHT: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Keeps only ASCII letters, upper-cased.
pub fn normalize(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

/// A permutation of the 26 letters with O(1) lookups in both directions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alphabet {
    letters: [u8; 26],
    pos: [u8; 26],
}

impl Alphabet {
    pub fn straight() -> Alphabet {
        STRAIGHT.parse().expect("straight alphabet")
    }

    /// Keyword-mixed alphabet: the keyword's distinct letters in order, then the remaining
    /// letters of the straight alphabet.
    pub fn keyword(keyword: &str) -> Alphabet {
        let mut used = [false; 26];
        let mut s = String::with_capacity(26);
        for b in normalize(keyword).bytes() {
            let k = (b - b'A') as usize;
            if !used[k] {
                used[k] = true;
                s.push(b as char);
            }
        }
        for (k, u) in used.iter().enumerate() {
            if !u {
                s.push((b'A' + k as u8) as char);
            }
        }
        s.parse().expect("keyword alphabet is a permutation")
    }

    /// Keyword alphabet written into rows of `width` and read out by columns
    /// (the "columnar" keyed-alphabet style).
    pub fn keyword_columnar(keyword: &str) -> Alphabet {
        let base = Alphabet::keyword(keyword).to_string();
        let width = normalize(keyword)
            .bytes()
            .fold(Vec::<u8>::new(), |mut v, b| {
                if !v.contains(&b) {
                    v.push(b)
                }
                v
            })
            .len()
            .max(1);
        let bytes = base.as_bytes();
        let mut out = String::with_capacity(26);
        for col in 0..width {
            let mut i = col;
            while i < 26 {
                out.push(bytes[i] as char);
                i += width;
            }
        }
        out.parse()
            .expect("columnar keyword alphabet is a permutation")
    }

    pub fn letter(&self, i: usize) -> u8 {
        self.letters[i % 26]
    }

    /// Position of an upper-case letter in this alphabet.
    pub fn position(&self, c: u8) -> usize {
        self.pos[(c - b'A') as usize] as usize
    }

    pub fn is_straight(&self) -> bool {
        self.letters
            .iter()
            .enumerate()
            .all(|(i, &b)| b == b'A' + i as u8)
    }
}

impl std::str::FromStr for Alphabet {
    type Err = String;

    /// Builds an alphabet from a 26-letter permutation string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = normalize(s);
        if s.len() != 26 {
            return Err(format!("alphabet must have 26 letters, got {}", s.len()));
        }

        let mut letters = [0u8; 26];
        let mut pos = [255u8; 26];
        for (i, b) in s.bytes().enumerate() {
            let k = (b - b'A') as usize;
            if pos[k] != 255 {
                return Err(format!("letter {} repeats", b as char));
            }
            pos[k] = i as u8;
            letters[i] = b;
        }
        Ok(Self { letters, pos })
    }
}

impl std::fmt::Display for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in &self.letters {
            write!(f, "{}", b as char)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_alphabet_dedupes_and_completes() {
        assert_eq!(
            Alphabet::keyword("SECRET").to_string(),
            "SECRTABDFGHIJKLMNOPQUVWXYZ"
        );
        assert_eq!(Alphabet::keyword("").to_string(), STRAIGHT);
        let a = Alphabet::keyword("SECRET");
        assert_eq!(a.position(b'S'), 0);
        assert_eq!(a.letter(4), b'T');
    }

    #[test]
    fn columnar_keyword_alphabet() {
        // KEY → KEYABCDFGHIJLMNOPQRSTUVWXZ in rows of 3:
        // K E Y / A B C / D F G / H I J / L M N / O P Q / R S T / U V W / X Z
        assert_eq!(
            Alphabet::keyword_columnar("KEY").to_string(),
            "KADHLORUXEBFIMPSVZYCGJNQTW"
        );
    }

    #[test]
    fn rejects_bad_alphabets() {
        assert!("ABC".parse::<Alphabet>().is_err());
        assert!("AABCDEFGHIJKLMNOPQRSTUVWXY".parse::<Alphabet>().is_err());
    }
}
