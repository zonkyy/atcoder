use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        k: usize,
    };

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<_>>();
    let ans = alphabet[0..k].iter().collect::<String>();
    println!("{}", ans);
}
