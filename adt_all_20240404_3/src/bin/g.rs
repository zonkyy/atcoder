use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    const UPPER: usize = 2 * 10usize.pow(5) + 1;
    let mut v = vec![0; UPPER];

    for i in a.iter() {
        v[*i] += 1;
    }
    for i in 1..UPPER {
        v[i] += v[i - 1];
    }

    let mut ans = 0;
    for i in 0..n {
        ans += v[a[i] - 1] * (n - v[a[i]]);
    }

    println!("{}", ans);
}
