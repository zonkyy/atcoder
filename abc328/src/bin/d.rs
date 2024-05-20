use std::{cmp, collections::BTreeSet};

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let mut set = (0..s.len()).collect::<BTreeSet<_>>();
    let mut idx = 0;
    while set.len() >= 2 && idx < set.len() - 2 {
        let a = *set.iter().nth(idx).unwrap();
        let b = *set.iter().nth(idx + 1).unwrap();
        let c = *set.iter().nth(idx + 2).unwrap();
        if s[a] == 'A' && s[b] == 'B' && s[c] == 'C' {
            set.remove(&a);
            set.remove(&b);
            set.remove(&c);
            idx = if idx >= 2 { idx - 2 } else { 0 };
        } else {
            idx += 1;
        }
    }

    for &i in set.iter() {
        print!("{}", s[i]);
    }
    if set.len() > 0 {
        println!();
    }
}
