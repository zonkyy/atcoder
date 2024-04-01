use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: String,
    };

    let mut substrs = HashSet::new();
    for l in 0..s.len() {
        for r in l..s.len() {
            substrs.insert(s.get(l..r + 1).unwrap());
        }
    }
    println!("{:?}", substrs.len());
}
