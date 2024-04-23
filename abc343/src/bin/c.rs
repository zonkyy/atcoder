use std::usize::MAX;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    const UPPER: usize = 10usize.pow(18);

    let mut v = Vec::new();
    let mut i: usize = 1;
    while i.pow(3) <= UPPER {
        let n = i.pow(3);
        if n.to_string() == n.to_string().chars().rev().collect::<String>() {
            v.push(n);
        }
        i += 1;
    }
    v.push(MAX);

    i = 0;
    while v[i] <= n {
        i += 1;
    }
    println!("{}", v[i - 1]);
}
