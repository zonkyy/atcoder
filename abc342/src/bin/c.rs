use std::collections::HashMap;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
        cd: [(char, char); q],
    };

    let mut place = HashMap::new();
    for (i, &c) in s.iter().enumerate() {
        place.entry(c).or_insert(vec![]).push(i);
    }

    let mut mut_s = s.clone();
    for &(c, d) in &cd {
        if let Some(entry) = place.get(&c) {
            let indices = entry.clone();
            for i in indices {
                mut_s[i] = d;
                place.entry(d).or_insert(vec![]).push(i);
            }
            place.remove(&c);
        }
    }

    println!("{}", mut_s.iter().collect::<String>());
}
