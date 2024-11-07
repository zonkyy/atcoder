use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    for si in 1..=s.len() {
        for ti in 1..=t.len() {
            dp[si][ti] = *[
                dp[si - 1][ti - 1] + ((s[si - 1] == t[ti - 1]) as usize),
                dp[si - 1][ti],
                dp[si][ti - 1],
            ]
            .iter()
            .max()
            .unwrap();
        }
    }

    println!("{:?}", dp[s.len()][t.len()]);
}
