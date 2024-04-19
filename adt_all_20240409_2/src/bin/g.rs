use std::collections::HashMap;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut k: i64,
        a: [i64; n],
    };

    let mut acc = vec![0];
    for i in 0..n {
        acc.push(acc[i] + a[i]);
    }

    let mut ans: usize = 0;
    let mut cnt = HashMap::new();
    for i in acc {
        if let Some(n) = cnt.get(&(i - k)) {
            ans += n;
        }
        *cnt.entry(i).or_insert(0) += 1;
    }

    println!("{}", ans);
}
