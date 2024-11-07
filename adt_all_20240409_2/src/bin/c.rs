use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    };

    println!("{}", if s.contains(&t) { "Yes" } else { "No" });
}
