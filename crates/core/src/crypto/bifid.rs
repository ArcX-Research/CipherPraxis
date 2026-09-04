//! Bifid: Polybius-square fractionation with a period (Delastelle).

use super::alphabet::{normalize, Alphabet};

/// A 5×5 Polybius square over 25 letters (J merged into I).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polybius {
    cells: [u8; 25],
    pos: [u8; 26],
}

impl Polybius {
    pub fn from_keyword(keyword: &str) -> Polybius {
        let a = Alphabet::keyword(&format!("{}{}", normalize(keyword).replace('J', "I"), ""));
        let mut cells = [0u8; 25];
        let mut pos = [0u8; 26];
        let mut i = 0;
        for k in 0..26 {
            let b = a.letter(k);
            if b == b'J' {
                continue;
            }
            cells[i] = b;
            pos[(b - b'A') as usize] = i as u8;
            i += 1;
        }
        pos[(b'J' - b'A') as usize] = pos[(b'I' - b'A') as usize];
        Polybius { cells, pos }
    }

    /// (row, column) coordinates, 1-based.
    pub fn coords(&self, c: u8) -> (u8, u8) {
        let p = self.pos[(c - b'A') as usize];
        (p / 5 + 1, p % 5 + 1)
    }

    pub fn letter(&self, row: u8, col: u8) -> u8 {
        self.cells[((row - 1) * 5 + (col - 1)) as usize]
    }

    pub fn rows(&self) -> Vec<String> {
        self.cells
            .chunks(5)
            .map(|r| r.iter().map(|&b| b as char).collect())
            .collect()
    }
}

pub fn encrypt(plain: &str, square: &Polybius, period: usize) -> String {
    let t = normalize(plain).replace('J', "I");
    let period = if period == 0 { t.len().max(1) } else { period };
    let mut out = String::with_capacity(t.len());
    for chunk in t.as_bytes().chunks(period) {
        let (rows, cols): (Vec<u8>, Vec<u8>) = chunk.iter().map(|&b| square.coords(b)).unzip();
        let seq: Vec<u8> = rows.into_iter().chain(cols).collect();
        for pair in seq.chunks(2) {
            out.push(square.letter(pair[0], pair[1]) as char);
        }
    }
    out
}

pub fn decrypt(cipher: &str, square: &Polybius, period: usize) -> String {
    let t = normalize(cipher).replace('J', "I");
    let period = if period == 0 { t.len().max(1) } else { period };
    let mut out = String::with_capacity(t.len());
    for chunk in t.as_bytes().chunks(period) {
        let seq: Vec<u8> = chunk
            .iter()
            .flat_map(|&b| {
                let (r, c) = square.coords(b);
                [r, c]
            })
            .collect();
        let n = chunk.len();
        for i in 0..n {
            out.push(square.letter(seq[i], seq[n + i]) as char);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delastelle_example() {
        let sq = Polybius::from_keyword("BGWKZQPNDSIOAXEFCLUMTHYVR");
        assert_eq!(sq.rows()[0], "BGWKZ");
        assert_eq!(encrypt("FLEEATONCE", &sq, 0), "UAEOLWRINS");
        assert_eq!(decrypt("UAEOLWRINS", &sq, 0), "FLEEATONCE");
    }

    #[test]
    fn periodic_roundtrip() {
        let sq = Polybius::from_keyword("PLAYFAIR");
        let c = encrypt("THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG", &sq, 5);
        assert_eq!(decrypt(&c, &sq, 5), "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG");
    }
}
