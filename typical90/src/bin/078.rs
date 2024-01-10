use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut map: HashMap<usize, usize> = HashMap::new();

    for (a, b) in ab {
        let mut max = a;
        if a < b {
            max = b;
        }
        *map.entry(max).or_insert(0) += 1;
    }

    let ans: usize = map.values().filter(|i| **i == 1).sum();
    println!("{}", ans);
}
