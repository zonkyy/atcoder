use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: String,
    };

    println!("{}", s.split(".").collect::<Vec<_>>().last().unwrap());
}
