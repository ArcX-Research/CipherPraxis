//! Affine and Caesar substitution over Z26.

use super::alphabet::normalize;
use super::numtheory::{gcd, mod_inverse};

pub fn is_unit(a: i64) -> bool {
    gcd(a, 26) == 1
}

/// c = a·p + b (mod 26); `a` must be a unit.
pub fn encrypt(plain: &str, a: i64, b: i64) -> Result<String, String> {
    if !is_unit(a) {
        return Err(format!(
            "a = {a} has no inverse modulo 26. Choose a value with gcd(a, 26) = 1."
        ));
    }
    Ok(normalize(plain)
        .bytes()
        .map(|c| (b'A' + ((a * (c - b'A') as i64 + b).rem_euclid(26)) as u8) as char)
        .collect())
}

/// p = a⁻¹·(c − b) (mod 26).
pub fn decrypt(cipher: &str, a: i64, b: i64) -> Result<String, String> {
    let inv = mod_inverse(a, 26).ok_or_else(|| format!("a = {a} has no inverse modulo 26."))?;
    Ok(normalize(cipher)
        .bytes()
        .map(|c| (b'A' + ((inv * ((c - b'A') as i64 - b)).rem_euclid(26)) as u8) as char)
        .collect())
}

pub fn caesar(text: &str, shift: i64) -> String {
    encrypt(text, 1, shift).expect("1 is a unit")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_vector() {
        assert_eq!(encrypt("AFFINE CIPHER", 5, 8).unwrap(), "IHHWVCSWFRCP");
        assert_eq!(decrypt("IHHWVCSWFRCP", 5, 8).unwrap(), "AFFINECIPHER");
        assert!(encrypt("A", 13, 0).is_err());
        assert_eq!(caesar("XYZ", 3), "ABC");
    }
}
