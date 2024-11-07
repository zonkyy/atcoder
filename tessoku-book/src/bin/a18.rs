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

    if dp[n - 1][s] {
        println!("Yes");
    } else {
        println!("No");
    }
}
