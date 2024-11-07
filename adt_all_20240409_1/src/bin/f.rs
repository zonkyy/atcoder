use std::cmp;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        fs: [(Usize1, usize); n],
    };

    let UPPER = 3 * 10usize.pow(5);
    let mut top2 = vec![[0, 0]; UPPER];
    for (f, s) in fs {
        if s >= top2[f][0] {
            top2[f][1] = top2[f][0];
            top2[f][0] = s;
        } else if s > top2[f][1] {
            top2[f][1] = s;
        }
    }

    top2.sort_by(|a, b| b[0].cmp(&a[0]).then(b[1].cmp(&a[1])));
    let ans = cmp::max(top2[0][0] + top2[1][0], top2[0][0] + top2[0][1] / 2);
    println!("{}", ans);
}
