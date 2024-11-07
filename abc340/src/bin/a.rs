use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        d: usize,
    };

    let ans = (a..=b).step_by(d).map(|x| x.to_string()).join(" ");
    println!("{}", ans);
}
