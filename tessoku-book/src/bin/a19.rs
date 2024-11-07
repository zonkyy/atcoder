use std::cmp;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    };

    let mut dp = vec![vec![0; w + 1]; n + 1];
    for r in 0..n {
        let (ww, vv) = wv[r];
        for c in 0..=w {
            dp[r + 1][c] = cmp::max(dp[r + 1][c], dp[r][c]);
            if c + ww < w + 1 {
                dp[r + 1][c + ww] = cmp::max(dp[r + 1][c + ww], dp[r][c] + vv);
            }
        }
    }

    println!("{}", dp[n][w]);
}
