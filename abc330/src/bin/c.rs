use std::collections::BTreeSet;

use proconio::{fastout, input, marker::*};

fn val(x: i64, y: i64) -> i64 {
    return x * x + y * y;
}

#[fastout]
fn main() {
    input! {
        d: i64,
    };

    let mut min = i64::MAX;
    let mut x = 0;
    let mut y = 0;
    while val(x, y) <= d {
        min = min.min((d - val(x, y)).abs());
        y += 1;
    }

    while x <= y {
        if val(x, y) < d {
            x += 1;
        } else {
            y -= 1;
        }
        min = min.min((d - val(x, y)).abs());
    }

    println!("{}", min);
}
