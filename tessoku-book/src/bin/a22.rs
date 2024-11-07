use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n-1],
        b: [Usize1; n-1],
    };

    let mut dp: Vec<usize> = vec![0; n];
    for i in 0..(n - 1) {
        dp[a[i]] = dp[a[i]].max(dp[i] + 100);
        dp[b[i]] = dp[b[i]].max(dp[i] + 150);
    }

    println!("{}", dp[n - 1]);
}
