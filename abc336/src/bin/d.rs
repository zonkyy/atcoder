use std::cmp;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut n: usize,
        mut a: [usize; n],
    };

    let mut lv = vec![0; n + 2];
    for i in 1..(n + 1) {
        lv[i] = cmp::min(lv[i - 1] + 1, a[i - 1]);
    }
    let mut rv = vec![0; n + 2];
    for i in (1..(n + 1)).rev() {
        rv[i] = cmp::min(rv[i + 1] + 1, a[i - 1])
    }

    let mut ans = 0;
    for (&l, &r) in lv.iter().zip(rv.iter()) {
        ans = ans.max(cmp::min(l, r));
    }

    println!("{}", ans);
}
