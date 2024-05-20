use std::cmp;

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    };

    let mut ans = Vec::new();
    for &i in a.iter() {
        let m = cmp::max(r - i, l - i);
        if i <= l {
            ans.push(l);
        } else if r <= i {
            ans.push(r);
        } else {
            ans.push(i);
        }
    }

    println!("{}", ans.iter().join(" "));
}
