use std::cmp;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d],
    };

    let mut l_acc = vec![0; n + 2];
    let mut r_acc = vec![0; n + 2];
    for i in 0..n {
        l_acc[i + 1] = cmp::max(l_acc[i], a[i]);
        r_acc[n - i] = cmp::max(r_acc[n - i + 1], a[n - i - 1]);
    }

    for (l, r) in lr {
        let lmax = l_acc[l - 1];
        let rmax = r_acc[r + 1];
        println!("{}", cmp::max(lmax, rmax));
    }
}
