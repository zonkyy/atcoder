use std::cmp;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    };

    // let mut cost = vec![0; n];
    // cost[1] = a[0];
    // for i in 2..n {
    //     cost[i] = cmp::min(cost[i - 1] + a[i - 1], cost[i - 2] + b[i - 2]);
    // }

    // println!("{}", cost[n - 1]);

    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;
    for i in 0..(n - 2) {
        dp[i + 1] = dp[i + 1].min(dp[i] + a[i]);
        dp[i + 2] = dp[i + 2].min(dp[i] + b[i]);
    }
    dp[n - 1] = dp[n - 1].min(dp[n - 2] + a[n - 2]);

    println!("{}", dp[n - 1]);
}
