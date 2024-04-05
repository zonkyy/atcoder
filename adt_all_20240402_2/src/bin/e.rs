use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xk: [(usize, Usize1); q],
    };

    let mut table = HashMap::new();
    for i in 0..n {
        table.entry(a[i]).or_insert(Vec::new()).push(i + 1);
    }

    for (x, k) in xk {
        if let Some(r) = table.get(&x) {
            println!("{}", r.get(k).map(|&x| x as i64).unwrap_or(-1));
        } else {
            println!("-1");
        }
    }
}
