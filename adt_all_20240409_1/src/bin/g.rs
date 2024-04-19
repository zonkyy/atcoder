use std::cmp;

use proconio::{fastout, input, marker::*};

fn f(a: u128, b: u128) -> u128 {
    a * a * a + a * a * b + a * b * b + b * b * b
}

#[fastout]
fn main() {
    input! {
        n: u128,
    };

    let mut upper = 0;
    while upper * upper * upper < n {
        upper += 1;
    }
    let mut l = 0;
    let mut r = upper;
    let mut min = upper * upper * upper;
    while l < upper {
        while f(l, r) > n {
            r -= 1;
        }
        min = cmp::min(min, f(l, r + 1));

        while f(l, r) < n {
            l += 1;
        }
        min = cmp::min(min, f(l, r));

        if f(l, r) == n {
            min = n;
            break;
        }
    }

    println!("{}", min);
}
