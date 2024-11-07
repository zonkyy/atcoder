use std::cmp;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }

    for r in (0..n).rev() {
        for c in (r + 1)..n {
            dp[r][c] = cmp::max(dp[r + 1][c], dp[r][c - 1]);
            if s[r] == s[c] {
                dp[r][c] = dp[r][c].max(dp[r + 1][c - 1] + 2);
            }
        }
    }

    println!("{}", dp[0][n - 1]);
}
