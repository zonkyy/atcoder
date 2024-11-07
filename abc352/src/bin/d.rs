use std::collections::BTreeSet;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    };

    let mut q = vec![0; n];
    for (idx, &i) in p.iter().enumerate() {
        q[i - 1] = idx;
    }

    let mut buf = BTreeSet::new();
    for &i in &q[0..k] {
        buf.insert(i);
    }

    let mut ans = buf.last().unwrap() - buf.first().unwrap();
    for i in 0..(n - k) {
        buf.remove(&q[i]);
        buf.insert(q[i + k]);
        ans = ans.min(buf.last().unwrap() - buf.first().unwrap());
    }

    println!("{}", ans);
}
