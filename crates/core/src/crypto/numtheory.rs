//! Elementary number theory: gcd, modular inverses, the Chinese remainder theorem, totient.

pub fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Extended Euclid: returns (g, x, y) with a·x + b·y = g = gcd(a, b).
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a.abs(), if a < 0 { -1 } else { 1 }, 0);
    }
    let (g, x1, y1) = egcd(b, a.rem_euclid(b));
    (g, y1, x1 - (a.div_euclid(b)) * y1)
}

/// Multiplicative inverse of `a` modulo `m`, if it exists.
pub fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    if m <= 1 {
        return None;
    }
    let (g, x, _) = egcd(a.rem_euclid(m), m);
    if g != 1 {
        None
    } else {
        Some(x.rem_euclid(m))
    }
}

/// Units of Z_m (residues coprime to m).
pub fn units(m: i64) -> Vec<i64> {
    (1..m).filter(|&a| gcd(a, m) == 1).collect()
}

pub fn totient(m: i64) -> usize {
    units(m).len()
}

/// Chinese remainder theorem for congruences x ≡ rᵢ (mod mᵢ). Moduli need not be coprime;
/// returns `None` when the system is inconsistent. Result is (x, M) with 0 ≤ x < M.
pub fn crt(congruences: &[(i64, i64)]) -> Option<(i64, i64)> {
    let mut x: i128 = 0;
    let mut m: i128 = 1;
    for &(r, n) in congruences {
        if n <= 0 {
            return None;
        }
        let r = (r as i128).rem_euclid(n as i128);
        let n = n as i128;
        let g = gcd(m as i64, n as i64) as i128;
        if (r - x).rem_euclid(g) != 0 {
            return None;
        }
        let lcm = m / g * n;
        // Solve x + m·t ≡ r (mod n)  ⇒  (m/g)·t ≡ (r − x)/g (mod n/g)
        let n_g = n / g;
        if n_g > 1 {
            let inv = mod_inverse((m / g).rem_euclid(n_g) as i64, n_g as i64)? as i128;
            let t = ((r - x) / g * inv).rem_euclid(n_g);
            x = (x + m * t).rem_euclid(lcm);
        }
        // When n divides m the congruence is redundant (consistency was checked above).
        m = lcm;
    }
    Some((x as i64, m as i64))
}

/// Trial-division factorisation, smallest factor first.
pub fn factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut p = 2;
    while p * p <= n {
        let mut e = 0;
        while n % p == 0 {
            n /= p;
            e += 1;
        }
        if e > 0 {
            out.push((p, e));
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b) * b).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverses_mod_26() {
        assert_eq!(mod_inverse(7, 26), Some(15));
        assert_eq!(mod_inverse(3, 26), Some(9));
        assert_eq!(mod_inverse(13, 26), None);
        assert_eq!(units(26), vec![1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25]);
        assert_eq!(totient(26), 12);
    }

    #[test]
    fn crt_examples() {
        assert_eq!(crt(&[(2, 3), (3, 5), (2, 7)]), Some((23, 105)));
        assert_eq!(crt(&[(1, 4), (3, 6)]), Some((9, 12)));
        assert_eq!(crt(&[(1, 4), (2, 6)]), None);
        assert_eq!(crt(&[]), Some((0, 1)));
        // Redundant congruences (modulus divides the accumulated modulus) are consistent, not errors.
        assert_eq!(crt(&[(1, 4), (1, 2)]), Some((1, 4)));
        assert_eq!(crt(&[(3, 5), (3, 5)]), Some((3, 5)));
        assert_eq!(crt(&[(1, 4), (0, 2)]), None);
    }

    #[test]
    fn factor_and_lcm() {
        assert_eq!(factorize(360), vec![(2, 3), (3, 2), (5, 1)]);
        assert_eq!(lcm(7, 9), 63);
        assert_eq!(egcd(240, 46).0, 2);
    }
}
