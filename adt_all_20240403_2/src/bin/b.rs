use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    if n >= 212 {
        println!("8");
    } else if n >= 126 {
        println!("6");
    } else {
        println!("4");
    }
}
