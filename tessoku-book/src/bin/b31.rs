use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let ans = n / 3 + n / 5 + n / 7 - n / 15 - n / 35 - n / 21 + n / 105;
    println!("{}", ans);
}
