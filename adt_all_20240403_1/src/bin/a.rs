use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    println!("0{}", s[0..=2].iter().collect::<String>());
}
