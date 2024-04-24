use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut abx: [(usize, usize, Usize1); n-1],
    };
    const UPPER: usize = usize::MAX;
    abx.sort_by_key(|x| x.2);

    let mut dp = vec![UPPER; n];
    dp[0] = 0;

    for (i, &(a, b, x)) in abx.iter().enumerate() {
        dp[i + 1] = dp[i + 1].min(dp[i] + a);
        dp[x] = dp[x].min(dp[i] + b);
    }

    println!("{:?}", dp);
}
