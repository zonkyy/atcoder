use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut max_long = vec![0; 26];
    for (key, group) in &s.into_iter().group_by(|x| *x) {
        let idx = key as usize - 'a' as usize;
        let len = group.collect::<Vec<_>>().len();
        max_long[idx] = max_long[idx].max(len);
    }

    println!("{}", max_long.iter().sum::<usize>());
}
