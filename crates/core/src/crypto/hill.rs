//! Hill cipher: linear maps on letter blocks modulo 26 (2×2 and 3×3).

use super::alphabet::normalize;
use super::numtheory::mod_inverse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix {
    pub n: usize,
    pub data: Vec<i64>, // row-major, reduced mod 26
}

impl Matrix {
    pub fn new(n: usize, data: Vec<i64>) -> Result<Matrix, String> {
        if !(2..=3).contains(&n) {
            return Err("only 2×2 and 3×3 matrices are supported".into());
        }
        if data.len() != n * n {
            return Err(format!("expected {} entries, got {}", n * n, data.len()));
        }
        Ok(Matrix {
            n,
            data: data.into_iter().map(|x| x.rem_euclid(26)).collect(),
        })
    }

    pub fn get(&self, r: usize, c: usize) -> i64 {
        self.data[r * self.n + c]
    }

    pub fn det(&self) -> i64 {
        let d = match self.n {
            2 => self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0),
            _ => {
                self.get(0, 0) * (self.get(1, 1) * self.get(2, 2) - self.get(1, 2) * self.get(2, 1))
                    - self.get(0, 1)
                        * (self.get(1, 0) * self.get(2, 2) - self.get(1, 2) * self.get(2, 0))
                    + self.get(0, 2)
                        * (self.get(1, 0) * self.get(2, 1) - self.get(1, 1) * self.get(2, 0))
            }
        };
        d.rem_euclid(26)
    }

    /// Inverse modulo 26 via the adjugate; `None` when det is not a unit.
    pub fn inverse(&self) -> Option<Matrix> {
        let det_inv = mod_inverse(self.det(), 26)?;
        let n = self.n;
        let mut adj = vec![0i64; n * n];
        for r in 0..n {
            for c in 0..n {
                // cofactor of (c, r) goes to (r, c)
                let minor = self.minor(c, r);
                let sign = if (r + c) % 2 == 0 { 1 } else { -1 };
                adj[r * n + c] = (sign * minor * det_inv).rem_euclid(26);
            }
        }
        Some(Matrix { n, data: adj })
    }

    fn minor(&self, row: usize, col: usize) -> i64 {
        let n = self.n;
        if n == 2 {
            return self.get(1 - row, 1 - col);
        }
        let rows: Vec<usize> = (0..3).filter(|&r| r != row).collect();
        let cols: Vec<usize> = (0..3).filter(|&c| c != col).collect();
        self.get(rows[0], cols[0]) * self.get(rows[1], cols[1])
            - self.get(rows[0], cols[1]) * self.get(rows[1], cols[0])
    }

    fn apply(&self, block: &[i64]) -> Vec<i64> {
        (0..self.n)
            .map(|r| {
                (0..self.n)
                    .map(|c| self.get(r, c) * block[c])
                    .sum::<i64>()
                    .rem_euclid(26)
            })
            .collect()
    }
}

/// Encrypts blocks of `n` letters; pads with `X`.
pub fn encrypt(plain: &str, key: &Matrix) -> String {
    let mut t = normalize(plain);
    while t.len() % key.n != 0 {
        t.push('X');
    }
    t.as_bytes()
        .chunks(key.n)
        .flat_map(|chunk| {
            let block: Vec<i64> = chunk.iter().map(|&b| (b - b'A') as i64).collect();
            key.apply(&block)
                .into_iter()
                .map(|v| (b'A' + v as u8) as char)
        })
        .collect()
}

pub fn decrypt(cipher: &str, key: &Matrix) -> Result<String, String> {
    let inv = key
        .inverse()
        .ok_or_else(|| format!("determinant {} is not a unit modulo 26", key.det()))?;
    Ok(encrypt(cipher, &inv))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_by_two_vector() {
        let k = Matrix::new(2, vec![3, 3, 2, 5]).unwrap();
        assert_eq!(encrypt("HELP", &k), "HIAT");
        assert_eq!(decrypt("HIAT", &k).unwrap(), "HELP");
        assert_eq!(k.inverse().unwrap().data, vec![15, 17, 20, 9]);
    }

    #[test]
    fn three_by_three_vector() {
        let k = Matrix::new(3, vec![6, 24, 1, 13, 16, 10, 20, 17, 15]).unwrap();
        assert_eq!(encrypt("ACT", &k), "POH");
        assert_eq!(decrypt("POH", &k).unwrap(), "ACT");
        assert_eq!(
            k.inverse().unwrap().data,
            vec![8, 5, 10, 21, 8, 21, 21, 12, 8]
        );
    }

    #[test]
    fn singular_matrix_has_no_inverse() {
        let k = Matrix::new(2, vec![2, 4, 1, 2]).unwrap();
        assert!(k.inverse().is_none());
        assert!(decrypt("AB", &k).is_err());
    }
}
