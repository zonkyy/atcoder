use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    };
    const N_MAX: usize = 69;
    const A_MAX: usize = 10009;

    let mut dp = vec![vec![false; A_MAX]; N_MAX];
    dp[0][0] = true;
    dp[0][a[0]] = true;

    for i in 1..n {
        for j in 0..A_MAX {
            if dp[i - 1][j] {
                dp[i][j] = true;
            }
            if a[i] <= j && dp[i - 1][j - a[i]] {
                dp[i][j] = true;
            }
        }
    }

    if !dp[n - 1][s] {
        println!("-1");
        return;
    }

    let mut si = s;
    let mut route = Vec::new();
    for i in (1..n).rev() {
        if a[i] <= si && dp[i - 1][si - a[i]] {
            route.push(i + 1);
            si -= a[i];
        }
    }
    if si > 0 {
        route.push(1);
    }

    println!("{}", route.len());
    println!("{}", route.iter().rev().join(" "));
}
