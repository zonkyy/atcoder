use std::collections::{BTreeMap, HashMap};

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
        cd: [(char, char); q],
    };

    let mut table = BTreeMap::new();
    for c in 'a'..='z' {
        table.insert(c, c);
    }

    for &(c, d) in cd.iter() {
        for (k, v) in table.clone() {
            if v == c {
                *table.entry(k).or_default() = d;
            }
        }
    }

    for c in s {
        print!("{}", table[&c]);
    }
    println!();
}
