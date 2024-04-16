use std::collections::BTreeMap;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(usize, usize); n],
    };

    let mut row_map = BTreeMap::new();
    let mut col_map = BTreeMap::new();
    let mut i = 0;
    for (a, b) in ab {
        row_map.entry(a).or_insert(vec![]).push(i);
        col_map.entry(b).or_insert(vec![]).push(i);
        i += 1;
    }

    let mut ans = vec![(0, 0); n];
    for (idx, (_, v)) in row_map.iter_mut().enumerate() {
        for vv in v {
            ans[*vv].0 = idx;
        }
    }
    for (idx, (_, v)) in col_map.iter_mut().enumerate() {
        for vv in v {
            ans[*vv].1 = idx;
        }
    }

    for (r, c) in ans {
        println!("{} {}", r + 1, c + 1);
    }
}
