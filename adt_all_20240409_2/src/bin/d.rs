use std::collections::HashMap;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let mut cnt = HashMap::new();
    for name in s {
        *cnt.entry(name).or_insert(0) += 1;
    }

    // cnt から value が最大の key を取り出す
    let ans = cnt.iter().max_by_key(|&(_, v)| v).unwrap().0;

    println!("{}", ans);
}
