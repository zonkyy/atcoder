use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    };

    if a + b > 0 {
        println!("{}", 0);
    } else {
        println!("{}", 9);
    }
}
