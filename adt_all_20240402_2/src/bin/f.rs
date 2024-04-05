use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
    };

    'outer: for perm in s.into_iter().permutations(n) {
        for window in perm.windows(2) {
            if window[0]
                .chars()
                .zip(window[1].chars())
                .filter(|(a, b)| a != b)
                .count()
                != 1
            {
                continue 'outer;
            }
        }
        println!("Yes");
        return;
    }

    println!("No");
}
