//! Statistical instruments: letter frequencies, index of coincidence, periodic and lag
//! coincidence profiles, Kasiski repeats, chi-squared fits and shuffled-null z-scores.

use super::alphabet::normalize;
use super::rng::Rng;

/// English letter frequencies (A–Z), from standard corpus tables.
pub const ENGLISH: [f64; 26] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
    0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
    0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074,
];

pub fn english_ic() -> f64 {
    ENGLISH.iter().map(|p| p * p).sum()
}

pub const UNIFORM_IC: f64 = 1.0 / 26.0;

pub fn counts(text: &str) -> [usize; 26] {
    let mut c = [0usize; 26];
    for b in normalize(text).bytes() {
        c[(b - b'A') as usize] += 1;
    }
    c
}

/// Index of coincidence Σ nᵢ(nᵢ−1) / (n(n−1)); 0 for fewer than two letters.
pub fn ic_of_counts(c: &[usize; 26]) -> f64 {
    let n: usize = c.iter().sum();
    if n < 2 {
        return 0.0;
    }
    c.iter()
        .map(|&k| (k * k.saturating_sub(1)) as f64)
        .sum::<f64>()
        / (n * (n - 1)) as f64
}

pub fn index_of_coincidence(text: &str) -> f64 {
    ic_of_counts(&counts(text))
}

/// Exact standard deviation of the IC estimator under independent draws from `p`, for a
/// sample of `n` letters (delta-free exact variance of the U-statistic).
pub fn ic_std_dev(p: &[f64; 26], n: usize) -> f64 {
    if n < 2 {
        return 0.0;
    }
    let s2: f64 = p.iter().map(|x| x * x).sum();
    let s3: f64 = p.iter().map(|x| x * x * x).sum();
    let n = n as f64;
    let var = (2.0 / (n * (n - 1.0))) * (s2 - s2 * s2)
        + (4.0 * (n - 2.0) / (n * (n - 1.0))) * (s3 - s2 * s2);
    var.max(0.0).sqrt()
}

/// Mean IC over the `m` cosets of the text.
pub fn periodic_ic(text: &str, m: usize) -> f64 {
    let t = normalize(text);
    let m = m.max(1);
    let mut cs = vec![[0usize; 26]; m];
    for (i, b) in t.bytes().enumerate() {
        cs[i % m][(b - b'A') as usize] += 1;
    }
    cs.iter().map(ic_of_counts).sum::<f64>() / m as f64
}

/// (period, mean coset IC) for periods 1..=max_m.
pub fn ic_scan(text: &str, max_m: usize) -> Vec<(usize, f64)> {
    (1..=max_m.max(1))
        .map(|m| (m, periodic_ic(text, m)))
        .collect()
}

/// Fraction of positions i with cᵢ = cᵢ₊ₗ.
pub fn coincidence_at_lag(text: &str, lag: usize) -> f64 {
    let t = normalize(text).into_bytes();
    if lag == 0 || t.len() <= lag {
        return 0.0;
    }
    let hits = (0..t.len() - lag).filter(|&i| t[i] == t[i + lag]).count();
    hits as f64 / (t.len() - lag) as f64
}

pub fn lag_profile(text: &str, max_lag: usize) -> Vec<(usize, f64)> {
    (1..=max_lag.max(1))
        .map(|l| (l, coincidence_at_lag(text, l)))
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct Repeat {
    pub gram: String,
    pub positions: Vec<usize>,
    pub distances: Vec<usize>,
}

/// Kasiski examination: repeated n-grams (length ≥ `min_len`) with their positions and the
/// distances between consecutive occurrences.
pub fn kasiski(text: &str, min_len: usize, max_len: usize) -> Vec<Repeat> {
    let t = normalize(text);
    let bytes = t.as_bytes();
    let mut out = Vec::new();
    for len in (min_len.max(2)..=max_len.max(min_len)).rev() {
        if bytes.len() < len {
            continue;
        }
        let mut seen: std::collections::BTreeMap<&[u8], Vec<usize>> =
            std::collections::BTreeMap::new();
        for i in 0..=bytes.len() - len {
            seen.entry(&bytes[i..i + len]).or_default().push(i);
        }
        for (g, pos) in seen {
            if pos.len() < 2 {
                continue;
            }
            let gram = std::str::from_utf8(g).unwrap().to_string();
            // Skip grams contained in an already-reported longer repeat at the same positions.
            if out
                .iter()
                .any(|r: &Repeat| r.gram.contains(&gram) && r.positions.len() == pos.len())
            {
                continue;
            }
            let distances = pos.windows(2).map(|w| w[1] - w[0]).collect();
            out.push(Repeat {
                gram,
                positions: pos,
                distances,
            });
        }
    }
    out.sort_by(|a, b| {
        b.positions
            .len()
            .cmp(&a.positions.len())
            .then(b.gram.len().cmp(&a.gram.len()))
            .then(a.gram.cmp(&b.gram))
    });
    out
}

/// How many Kasiski distances each candidate period divides.
pub fn factor_votes(repeats: &[Repeat], max_period: usize) -> Vec<(usize, usize)> {
    (2..=max_period.max(2))
        .map(|p| {
            (
                p,
                repeats
                    .iter()
                    .flat_map(|r| r.distances.iter())
                    .filter(|&&d| d % p == 0)
                    .count(),
            )
        })
        .collect()
}

/// χ² of observed counts against expected proportions.
pub fn chi_squared(observed: &[usize; 26], expected: &[f64; 26]) -> f64 {
    let n: usize = observed.iter().sum();
    if n == 0 {
        return 0.0;
    }
    observed
        .iter()
        .zip(expected)
        .map(|(&o, &e)| {
            let exp = e * n as f64;
            if exp > 0.0 {
                (o as f64 - exp).powi(2) / exp
            } else {
                0.0
            }
        })
        .sum()
}

/// Best Caesar shift of a (sub)text against English by χ²: returns (shift, χ²) with the
/// convention c = p + shift.
pub fn best_shift(observed: &[usize; 26]) -> (usize, f64) {
    let mut best = (0, f64::INFINITY);
    for s in 0..26 {
        let mut rotated = [0usize; 26];
        for (i, &o) in observed.iter().enumerate() {
            rotated[(i + 26 - s) % 26] = o;
        }
        let chi = chi_squared(&rotated, &ENGLISH);
        if chi < best.1 {
            best = (s, chi);
        }
    }
    best
}

/// Recovers a periodic additive key of length `m` coset by coset (Vigenère convention).
pub fn recover_shifts(text: &str, m: usize) -> Vec<usize> {
    let t = normalize(text);
    let m = m.max(1);
    let mut cs = vec![[0usize; 26]; m];
    for (i, b) in t.bytes().enumerate() {
        cs[i % m][(b - b'A') as usize] += 1;
    }
    cs.iter().map(|c| best_shift(c).0).collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct NullSummary {
    pub observed: f64,
    pub mean: f64,
    pub sd: f64,
    pub z: f64,
    pub trials: usize,
    /// Fraction of shuffled trials with a statistic ≥ the observed value.
    pub p_upper: f64,
}

/// Compares a statistic of the text against the same statistic on letter-shuffled copies.
pub fn shuffled_null<F: Fn(&str) -> f64>(
    text: &str,
    trials: usize,
    seed: u64,
    stat: F,
) -> NullSummary {
    let t = normalize(text);
    let observed = stat(&t);
    let mut rng = Rng::new(seed);
    let mut bytes = t.into_bytes();
    let mut values = Vec::with_capacity(trials);
    for _ in 0..trials {
        rng.shuffle(&mut bytes);
        values.push(stat(std::str::from_utf8(&bytes).unwrap()));
    }
    let n = values.len().max(1) as f64;
    let mean = values.iter().sum::<f64>() / n;
    let var = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0).max(1.0);
    let sd = var.sqrt();
    let z = if sd > 0.0 {
        (observed - mean) / sd
    } else {
        0.0
    };
    let p_upper = (values.iter().filter(|&&v| v >= observed).count() as f64 + 1.0) / (n + 1.0);
    NullSummary {
        observed,
        mean,
        sd,
        z,
        trials: values.len(),
        p_upper,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::periodic::{encrypt, Variant};

    const SAMPLE: &str = "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair, we had everything before us, we had nothing before us, we were all going direct to Heaven, we were all going direct the other way";

    #[test]
    fn ic_basics() {
        assert!((index_of_coincidence("AAAA") - 1.0).abs() < 1e-12);
        assert!((index_of_coincidence("ABCD")).abs() < 1e-12);
        assert!((english_ic() - 0.0655).abs() < 0.002);
        let ic = index_of_coincidence(SAMPLE);
        assert!(ic > 0.055 && ic < 0.085, "{ic}");
        assert!(ic_std_dev(&ENGLISH, 150) > 0.0);
    }

    #[test]
    fn periodic_ic_finds_period() {
        let c = encrypt(SAMPLE, "LEMON", Variant::Vigenere).unwrap();
        let scan = ic_scan(&c, 12);
        let at5 = scan[4].1;
        let flat: f64 = scan
            .iter()
            .filter(|(m, _)| *m != 5 && *m != 10)
            .map(|(_, v)| *v)
            .sum::<f64>()
            / 10.0;
        assert!(at5 > flat + 0.015, "at5 {at5} vs flat {flat}");
        assert!(coincidence_at_lag(&c, 5) > coincidence_at_lag(&c, 3));
    }

    #[test]
    fn kasiski_votes_for_period() {
        let c = encrypt(SAMPLE, "LEMON", Variant::Vigenere).unwrap();
        let reps = kasiski(&c, 3, 6);
        assert!(!reps.is_empty());
        let votes = factor_votes(&reps, 12);
        let v5 = votes.iter().find(|(p, _)| *p == 5).unwrap().1;
        let v7 = votes.iter().find(|(p, _)| *p == 7).unwrap().1;
        assert!(v5 > v7);
    }

    #[test]
    fn shift_recovery() {
        let c = encrypt(SAMPLE, "LEMON", Variant::Vigenere).unwrap();
        assert_eq!(recover_shifts(&c, 5), vec![11, 4, 12, 14, 13]);
    }

    #[test]
    fn shuffled_null_is_flat_for_random_order() {
        let c = encrypt(SAMPLE, "LEMON", Variant::Vigenere).unwrap();
        let s = shuffled_null(&c, 200, 1, |t| periodic_ic(t, 5));
        assert!(s.z > 4.0, "z = {}", s.z);
        assert!(s.p_upper < 0.02);
        let flat = shuffled_null(&c, 200, 1, |t| periodic_ic(t, 7));
        assert!(flat.z.abs() < 3.0, "z = {}", flat.z);
    }
}
