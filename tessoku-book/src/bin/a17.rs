use std::cmp;

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    };

    let mut cost = vec![0; n];
    let mut route = vec![0; n];
    cost[1] = a[0];
    for i in 2..n {
        if cost[i - 1] + a[i - 1] <= cost[i - 2] + b[i - 2] {
            cost[i] = cost[i - 1] + a[i - 1];
            route[i] = i - 1;
        } else {
            cost[i] = cost[i - 2] + b[i - 2];
            route[i] = i - 2;
        }
    }

    let mut i = n - 1;
    let mut my_route = vec![i + 1];
    while i != 0 {
        i = route[i];
        my_route.push(i + 1);
    }

    println!("{}", my_route.len());
    println!("{}", my_route.iter().rev().join(" "));
}
