use itertools::Itertools;
use std::collections::HashSet;

use num_integer::gcd;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    let mut s = HashSet::new();
    for perm in xy.iter().permutations(2) {
        let dx = perm[0].0 - perm[1].0;
        let dy = perm[0].1 - perm[1].1;
        let d = gcd(dx, dy);
        s.insert((dx / d, dy / d));
    }

    println!("{}", s.len());
}
