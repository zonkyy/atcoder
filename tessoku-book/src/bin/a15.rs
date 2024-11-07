use std::collections::HashMap;

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    let mut b = a.clone();
    b.sort();

    let mut memo = HashMap::new();
    let mut pre = 0;
    let mut cnt = 0;
    for i in 0..n {
        if b[i] == pre {
            memo.insert(b[i], cnt);
        } else {
            cnt += 1;
            memo.insert(b[i], cnt);
            pre = b[i];
        }
    }

    let mut ans = Vec::new();
    for i in 0..n {
        if let Some(&val) = memo.get(&a[i]) {
            ans.push(val);
        }
    }

    println!("{}", ans.iter().join(" "));
}
