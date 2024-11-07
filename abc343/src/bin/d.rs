use std::collections::{HashMap, HashSet};

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(Usize1, usize); t],
    };

    let mut score = vec![0; n];
    let mut cnt = HashMap::new();
    cnt.insert(0, n);
    for (a, b) in ab {
        let pre_score = score[a];
        let new_score = score[a] + b;
        score[a] = new_score;

        *cnt.entry(pre_score).or_default() -= 1;
        if cnt.get(&pre_score).unwrap() == &0 {
            cnt.remove(&pre_score);
        }

        *cnt.entry(new_score).or_default() += 1;

        println!("{}", cnt.len());
    }
}
