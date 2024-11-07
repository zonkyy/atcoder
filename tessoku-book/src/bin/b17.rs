use std::cmp;

use itertools::Itertools;
use num::abs;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [i64; n],
    };
    let mut cost = vec![0; n];
    let mut route = vec![0; n];
    cost[1] = abs(h[1] - h[0]);
    for i in 2..n {
        if cost[i - 2] + abs(h[i] - h[i - 2]) < cost[i - 1] + abs(h[i] - h[i - 1]) {
            cost[i] = cost[i - 2] + abs(h[i] - h[i - 2]);
            route[i] = i - 2;
        } else {
            cost[i] = cost[i - 1] + abs(h[i] - h[i - 1]);
            route[i] = i - 1;
        }
    }

    let mut i = n - 1;
    let mut ans = vec![i + 1];
    while i != 0 {
        i = route[i];
        ans.push(i + 1);
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().rev().join(" "));
}
