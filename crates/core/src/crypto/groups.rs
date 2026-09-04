//! Permutations with cycle structure, and the dihedral group acting on positions.

use super::numtheory::gcd;

/// A permutation of {0, …, n−1} in one-line notation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Perm {
    pub map: Vec<usize>,
}

impl Perm {
    pub fn identity(n: usize) -> Perm {
        Perm {
            map: (0..n).collect(),
        }
    }

    pub fn new(map: Vec<usize>) -> Result<Perm, String> {
        let n = map.len();
        let mut seen = vec![false; n];
        for &m in &map {
            if m >= n || seen[m] {
                return Err("Each point must appear exactly once.".into());
            }
            seen[m] = true;
        }
        Ok(Perm { map })
    }

    /// Parses cycle notation such as `(0 3 5)(1 4)` over `n` points (0-based).
    pub fn from_cycles(s: &str, n: usize) -> Result<Perm, String> {
        let mut map: Vec<usize> = (0..n).collect();
        for cycle in s
            .split(')')
            .map(|c| c.trim_start_matches(|ch: char| ch.is_whitespace() || ch == '('))
        {
            let pts: Vec<usize> = cycle
                .split(|c: char| c == ',' || c.is_whitespace())
                .filter(|p| !p.is_empty())
                .map(|p| {
                    p.parse::<usize>()
                        .map_err(|_| format!("`{p}` is not a valid point."))
                })
                .collect::<Result<_, _>>()?;
            if pts.is_empty() {
                continue;
            }
            if pts.iter().any(|&p| p >= n) {
                return Err(format!(
                    "A point is outside the range 0 to {}.",
                    n.saturating_sub(1)
                ));
            }
            for w in 0..pts.len() {
                map[pts[w]] = pts[(w + 1) % pts.len()];
            }
        }
        Perm::new(map)
    }

    pub fn n(&self) -> usize {
        self.map.len()
    }

    pub fn apply(&self, i: usize) -> usize {
        self.map[i]
    }

    /// Composition `self ∘ other` (apply `other` first).
    pub fn compose(&self, other: &Perm) -> Perm {
        Perm {
            map: other.map.iter().map(|&i| self.map[i]).collect(),
        }
    }

    pub fn inverse(&self) -> Perm {
        let mut inv = vec![0; self.n()];
        for (i, &m) in self.map.iter().enumerate() {
            inv[m] = i;
        }
        Perm { map: inv }
    }

    /// Disjoint cycles including fixed points, each starting at its smallest element.
    pub fn cycles(&self) -> Vec<Vec<usize>> {
        let mut seen = vec![false; self.n()];
        let mut out = Vec::new();
        for start in 0..self.n() {
            if seen[start] {
                continue;
            }
            let mut cyc = vec![start];
            seen[start] = true;
            let mut j = self.map[start];
            while j != start {
                seen[j] = true;
                cyc.push(j);
                j = self.map[j];
            }
            out.push(cyc);
        }
        out
    }

    /// Cycle lengths in decreasing order.
    pub fn cycle_type(&self) -> Vec<usize> {
        let mut t: Vec<usize> = self.cycles().iter().map(|c| c.len()).collect();
        t.sort_unstable_by(|a, b| b.cmp(a));
        t
    }

    pub fn order(&self) -> u64 {
        self.cycle_type().iter().fold(1u64, |acc, &l| {
            let l = l as u64;
            acc / gcd(acc as i64, l as i64) as u64 * l
        })
    }

    /// Conjugate `g · self · g⁻¹`: relabels the cycles by `g`.
    pub fn conjugate_by(&self, g: &Perm) -> Perm {
        g.compose(self).compose(&g.inverse())
    }

    pub fn cycle_notation(&self) -> String {
        let cyc: Vec<String> = self
            .cycles()
            .into_iter()
            .filter(|c| c.len() > 1)
            .map(|c| {
                format!(
                    "({})",
                    c.iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            })
            .collect();
        if cyc.is_empty() {
            "()".into()
        } else {
            cyc.concat()
        }
    }
}

/// Element of the dihedral group D_n acting on Z_n by x ↦ ε·x + k.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dihedral {
    pub n: usize,
    pub reflect: bool,
    pub shift: usize,
}

impl Dihedral {
    pub fn rotation(n: usize, k: usize) -> Dihedral {
        Dihedral {
            n,
            reflect: false,
            shift: k % n,
        }
    }

    /// The reflection x ↦ −x + k.
    pub fn reflection(n: usize, k: usize) -> Dihedral {
        Dihedral {
            n,
            reflect: true,
            shift: k % n,
        }
    }

    pub fn apply(&self, x: usize) -> usize {
        let x = x % self.n;
        if self.reflect {
            (self.n - x + self.shift) % self.n
        } else {
            (x + self.shift) % self.n
        }
    }

    /// `self ∘ other` (apply `other` first).
    pub fn compose(&self, other: &Dihedral) -> Dihedral {
        assert_eq!(self.n, other.n);
        let n = self.n;
        let shift = if self.reflect {
            (n - other.shift + self.shift) % n
        } else {
            (other.shift + self.shift) % n
        };
        Dihedral {
            n,
            reflect: self.reflect ^ other.reflect,
            shift,
        }
    }

    pub fn inverse(&self) -> Dihedral {
        if self.reflect {
            *self // reflections are involutions
        } else {
            Dihedral::rotation(self.n, (self.n - self.shift) % self.n)
        }
    }

    pub fn order(&self) -> usize {
        if self.reflect {
            2
        } else if self.shift == 0 {
            1
        } else {
            self.n / gcd(self.n as i64, self.shift as i64) as usize
        }
    }

    pub fn to_perm(&self) -> Perm {
        Perm {
            map: (0..self.n).map(|x| self.apply(x)).collect(),
        }
    }

    /// Group-word form: r^k or s·r^k, with r: x ↦ x+1 and s: x ↦ −x.
    pub fn word(&self) -> String {
        match (self.reflect, self.shift) {
            (false, 0) => "e".into(),
            (false, k) => format!("r^{k}"),
            (true, 0) => "s".into(),
            // x ↦ −x + k  =  r^k ∘ s
            (true, k) => format!("r^{k} s"),
        }
    }

    pub fn all(n: usize) -> Vec<Dihedral> {
        (0..n)
            .map(|k| Dihedral::rotation(n, k))
            .chain((0..n).map(|k| Dihedral::reflection(n, k)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycles_and_order() {
        let p = Perm::from_cycles("(0 3 5)(1 4)", 6).unwrap();
        assert_eq!(p.cycle_type(), vec![3, 2, 1]);
        assert_eq!(p.order(), 6);
        assert_eq!(p.cycle_notation(), "(0 3 5)(1 4)");
        assert_eq!(p.compose(&p.inverse()), Perm::identity(6));
        let g = Perm::from_cycles("(0 1)", 6).unwrap();
        assert_eq!(p.conjugate_by(&g).cycle_type(), p.cycle_type());
    }

    #[test]
    fn dihedral_group_law() {
        let n = 8;
        let els = Dihedral::all(n);
        assert_eq!(els.len(), 16);
        for a in &els {
            for b in &els {
                let via_perm = a.to_perm().compose(&b.to_perm());
                assert_eq!(a.compose(b).to_perm(), via_perm);
            }
            assert_eq!(a.compose(&a.inverse()), Dihedral::rotation(n, 0));
        }
        assert_eq!(Dihedral::rotation(48, 12).order(), 4);
        assert_eq!(Dihedral::reflection(48, 5).order(), 2);
        assert_eq!(Dihedral::reflection(8, 3).word(), "r^3 s");
    }
}
