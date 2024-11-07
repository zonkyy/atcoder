use num::abs;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    };
    let BIT_MAX = 2_i32.pow(n as u32) as usize;

    let mut distance = vec![vec![0.0; n]; n];
    for s in 0..n {
        for g in 0..n {
            distance[s][g] = (abs(xy[s].0 - xy[g].0).pow(2) as f64
                + abs(xy[s].1 - xy[g].1).pow(2) as f64)
                .sqrt();
        }
    }

    let mut dp = vec![vec![f64::MAX; n]; BIT_MAX];
    dp[0][0] = 0.0;
    for r in 0..BIT_MAX {
        for now_city in 0..n {
            let next_bits = r | (1 << now_city);
            for next_city in 0..n {
                let new_value = dp[r][now_city] + distance[now_city][next_city];
                dp[next_bits][next_city] = dp[next_bits][next_city].min(new_value);
            }
        }
    }

    println!("{}", dp[BIT_MAX - 1][0]);
}
