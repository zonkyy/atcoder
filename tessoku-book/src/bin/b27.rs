use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    };

    println!("{}", num::integer::lcm(a, b));
}
