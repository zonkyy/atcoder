use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        (h1, w1): (usize, usize),
        a: [[usize; w1]; h1],
        (h2, w2): (usize, usize),
        b: [[usize; w2]; h2],
    };

    for r in (0..h1).combinations(h2) {
        for c in (0..w1).combinations(w2) {
            let mut v = Vec::new();
            for ri in r.iter() {
                let mut v_row = Vec::new();
                for ci in c.iter() {
                    v_row.push(a[*ri][*ci]);
                }
                v.push(v_row);
            }

            if v == b {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
