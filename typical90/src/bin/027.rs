use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut members: HashSet<&String> = HashSet::new();
    for (i, name) in s.iter().enumerate() {
        if !members.contains(name) {
            members.insert(name);
            println!("{}", i + 1);
        }
    }
}
