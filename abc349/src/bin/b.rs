use std::collections::HashMap;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let mut cnt = HashMap::new();
    for c in s {
        *cnt.entry(c).or_insert(0) += 1;
    }

    let mut cnt2 = HashMap::new();
    for (_, v) in cnt {
        *cnt2.entry(v).or_insert(0) += 1;
    }

    for (_, v) in cnt2 {
        if v == 0 || v == 2 {
            continue;
        }
        println!("No");
        return;
    }
    println!("Yes");
}
