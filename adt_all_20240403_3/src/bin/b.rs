use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut n: usize,
    };

    if n >= 42 {
        n += 1;
    }
    println!("AGC{:03}", n);
}
