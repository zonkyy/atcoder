use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    };

    for (ri, row) in a.iter().enumerate() {
        let ans = row
            .into_iter()
            .enumerate()
            .filter(|(ci, c)| **c == 1)
            .map(|(ci, c)| (ci + 1).to_string())
            .collect::<Vec<String>>();

        println!("{}", ans.join(" "));
    }
}
