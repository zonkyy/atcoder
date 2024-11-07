use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    const LIMIT: usize = 100009;
    let mut dp = [false; LIMIT];

    let min = std::cmp::min(a, b);
    for i in min..=n {
        if !dp[i - a] {
            dp[i] = true;
        }

        if b <= i && !dp[i - b] {
            dp[i] = true;
        }
    }

    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
