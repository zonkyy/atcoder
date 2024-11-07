use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut ans = 0;
    let mut l = 0;
    for r in 1..n {
        while a[r] - a[l] > k {
            l += 1;
        }
        ans += r - l;
    }

    println!("{}", ans);
}
