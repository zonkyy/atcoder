use std::cmp;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    };
    const V_MAX: usize = 100009;

    let mut dp = vec![vec![usize::MAX; V_MAX]; n + 1];
    dp[0][0] = 0;

    for r in 0..n {
        let (ww, vv) = wv[r];
        for c in 0..V_MAX {
            if dp[r][c] < usize::MAX {
                dp[r + 1][c] = cmp::min(dp[r + 1][c], dp[r][c]);
                dp[r + 1][c + vv] = cmp::min(dp[r + 1][c + vv], dp[r][c] + ww);
            }
        }
    }

    let mut ans = 0;
    for i in 0..V_MAX {
        if dp[n][i] <= w {
            ans = ans.max(i);
        }
    }
    println!("{}", ans);
}
