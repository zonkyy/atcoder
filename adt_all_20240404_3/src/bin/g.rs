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

    let mut m = HashMap::new();
    for i in a {
        *m.entry(i).or_insert(0) += 1;
    }

    let mut ans = 0;
    for comb in m.keys().combinations(3) {
        let mut tmp = 1;
        for c in comb {
            tmp *= m[c];
        }
        ans += tmp;
    }

    println!("{}", ans);
}
