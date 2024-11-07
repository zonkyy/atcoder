use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    println!("{}", n / 3 + n / 5 - n / 15);
}
