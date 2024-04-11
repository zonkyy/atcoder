use proconio::{fastout, input, marker::*};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    let mut bets = BTreeMap::new();
    for i in 0..n {
        input! {
            c: usize,
            a: [usize; c],
        }
        bets.entry(c).or_insert(vec![]).push((i, a));
    }
    input! {
        c: usize,
    }
    for (k, v) in bets {
        if v.iter().any(|(p, x)| x.contains(&c)) {
            let mut ans = Vec::new();
            let mut cnt = 0;
            for (p, x) in v.iter() {
                if x.contains(&c) {
                    ans.push((p + 1).to_string());
                    cnt += 1;
                }
            }
            println!("{}", cnt);
            println!("{}", ans.join(" "));
            return;
        }
    }

    println!("0\n");
}
