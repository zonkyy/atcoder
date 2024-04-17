use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n-1],
    };

    let ans = -a.into_iter().sum::<i64>();
    println!("{}", ans);
}
