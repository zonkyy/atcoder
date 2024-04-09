use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let ans = s[s.len() / 2];
    println!("{}", ans);
}
