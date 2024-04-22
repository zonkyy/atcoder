use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        t: String,
        n: usize,
    };
    const UPPER: usize = 10000;

    let mut s = Vec::new();
    for i in 0..n {
        input! {
            a: usize,
            ss: [String; a],
        }
        s.push(ss);
    }

    let mut dp = vec![vec![UPPER; t.len() + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..(n + 1) {
        for j in 0..(t.len() + 1) {
            dp[i][j] = dp[i - 1][j];
        }

        for j in 0..t.len() {
            for ss in s[i - 1].iter() {
                if t[j..].starts_with(ss.as_str()) {
                    let min = itertools::min([
                        dp[i - 1][j] + 1,
                        dp[i - 1][j + ss.len()],
                        dp[i][j + ss.len()],
                    ])
                    .unwrap();
                    dp[i][j + ss.len()] = min;
                }
            }
        }
    }

    println!(
        "{}",
        if dp[n][t.len()] >= UPPER {
            "-1".to_string()
        } else {
            dp[n][t.len()].to_string()
        }
    );
}
