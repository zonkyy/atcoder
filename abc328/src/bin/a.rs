use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        s: [usize; n],
    };

    println!("{}", s.iter().filter(|&e| *e <= x).sum::<usize>());
}
