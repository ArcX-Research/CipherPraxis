//! Transposition ciphers: keyed columnar (regular and irregular) and rail fence.

use super::alphabet::normalize;

/// Column read-out order for a keyword: letters ranked alphabetically, ties broken left to right.
pub fn key_order(keyword: &str) -> Vec<usize> {
    let k: Vec<u8> = normalize(keyword).into_bytes();
    let mut idx: Vec<usize> = (0..k.len()).collect();
    idx.sort_by(|&a, &b| k[a].cmp(&k[b]).then(a.cmp(&b)));
    idx
}

/// Lays the text into rows of `width`; returns the columns (ragged when the last row is short).
pub fn columns(text: &str, width: usize) -> Vec<String> {
    let t = normalize(text);
    let width = width.max(1);
    let mut cols = vec![String::new(); width];
    for (i, c) in t.chars().enumerate() {
        cols[i % width].push(c);
    }
    cols
}

/// Irregular (unpadded) columnar transposition when `pad` is `None`; regular when a pad letter
/// is given.
pub fn columnar_encrypt(plain: &str, keyword: &str, pad: Option<char>) -> Result<String, String> {
    let order = key_order(keyword);
    if order.is_empty() {
        return Err("keyword must contain at least one letter".into());
    }
    let mut t = normalize(plain);
    if let Some(p) = pad {
        while t.len() % order.len() != 0 {
            t.push(p.to_ascii_uppercase());
        }
    }
    let cols = columns(&t, order.len());
    Ok(order.iter().map(|&c| cols[c].clone()).collect())
}

pub fn columnar_decrypt(cipher: &str, keyword: &str) -> Result<String, String> {
    let order = key_order(keyword);
    if order.is_empty() {
        return Err("keyword must contain at least one letter".into());
    }
    let t = normalize(cipher);
    let width = order.len();
    let n = t.len();
    let long = n % width; // number of columns with an extra letter (leftmost ones)
    let base = n / width;
    let mut cols = vec![String::new(); width];
    let mut pos = 0;
    for &c in &order {
        let len = base + usize::from(long != 0 && c < long);
        cols[c] = t[pos..pos + len].to_string();
        pos += len;
    }
    let mut out = String::with_capacity(n);
    for r in 0..=base {
        for col in cols.iter() {
            if let Some(ch) = col.chars().nth(r) {
                out.push(ch);
            }
        }
    }
    Ok(out)
}

/// Rail-fence (zig-zag) transposition.
pub fn rail_fence_encrypt(plain: &str, rails: usize) -> String {
    let t = normalize(plain);
    if rails < 2 {
        return t;
    }
    let mut lanes = vec![String::new(); rails];
    for (i, c) in t.chars().enumerate() {
        lanes[rail_index(i, rails)].push(c);
    }
    lanes.concat()
}

pub fn rail_fence_decrypt(cipher: &str, rails: usize) -> String {
    let t = normalize(cipher);
    if rails < 2 {
        return t;
    }
    let n = t.len();
    let mut counts = vec![0usize; rails];
    for i in 0..n {
        counts[rail_index(i, rails)] += 1;
    }
    let mut lanes: Vec<Vec<char>> = Vec::new();
    let mut pos = 0;
    for &c in &counts {
        lanes.push(t[pos..pos + c].chars().rev().collect());
        pos += c;
    }
    (0..n)
        .map(|i| lanes[rail_index(i, rails)].pop().unwrap())
        .collect()
}

fn rail_index(i: usize, rails: usize) -> usize {
    let cycle = 2 * (rails - 1);
    let r = i % cycle;
    if r < rails {
        r
    } else {
        cycle - r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn irregular_columnar_vector() {
        assert_eq!(key_order("ZEBRAS"), vec![4, 2, 1, 3, 5, 0]);
        let c = columnar_encrypt("WE ARE DISCOVERED FLEE AT ONCE", "ZEBRAS", None).unwrap();
        assert_eq!(c, "EVLNACDTESEAROFODEECWIREE");
        assert_eq!(
            columnar_decrypt(&c, "ZEBRAS").unwrap(),
            "WEAREDISCOVEREDFLEEATONCE"
        );
    }

    #[test]
    fn regular_columnar_roundtrip() {
        let c = columnar_encrypt("WEAREDISCOVEREDFLEEATONCE", "ZEBRAS", Some('Q')).unwrap();
        assert_eq!(c.len(), 30);
        assert_eq!(
            columnar_decrypt(&c, "ZEBRAS").unwrap(),
            "WEAREDISCOVEREDFLEEATONCEQQQQQ"
        );
    }

    #[test]
    fn rail_fence_vector() {
        let c = rail_fence_encrypt("WEAREDISCOVEREDFLEEATONCE", 3);
        assert_eq!(c, "WECRLTEERDSOEEFEAOCAIVDEN");
        assert_eq!(rail_fence_decrypt(&c, 3), "WEAREDISCOVEREDFLEEATONCE");
    }
}
