use std::collections::HashSet;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    };

    let mut sums = HashSet::new();
    for ai in a.iter() {
        for bi in b.iter() {
            for ci in c.iter() {
                sums.insert(ai + bi + ci);
            }
        }
    }

    for xi in x.iter() {
        println!("{}", if sums.contains(&xi) { "Yes" } else { "No" });
    }
}
