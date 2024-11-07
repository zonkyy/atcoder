use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };

    let ans = a.binary_search(&x);
    println!("{}", ans.unwrap() + 1);
}
