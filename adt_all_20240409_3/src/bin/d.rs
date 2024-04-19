use std::collections::HashSet;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    };

    let mut all = HashSet::new();
    for i in 0..n {
        for j in (i + 1)..n {
            all.insert((i + 1, j + 1));
        }
    }
    let mut good = HashSet::new();
    for i in 0..m {
        for j in 1..n {
            if a[i][j - 1] < a[i][j] {
                good.insert((a[i][j - 1], a[i][j]));
            } else {
                good.insert((a[i][j], a[i][j - 1]));
            }
        }
    }

    println!("{}", all.len() - good.len());
}
