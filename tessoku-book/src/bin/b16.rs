use std::cmp;

use num::abs;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [i64; n],
    };
    let mut cost = vec![0; n];
    cost[1] = abs(h[1] - h[0]);
    for i in 2..n {
        cost[i] = cmp::min(
            cost[i - 2] + abs(h[i] - h[i - 2]),
            cost[i - 1] + abs(h[i] - h[i - 1]),
        );
    }

    println!("{}", cost[n - 1]);
}
