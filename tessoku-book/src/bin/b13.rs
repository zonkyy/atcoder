use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    };

    let mut acc = vec![0];
    for i in 0..n {
        acc.push(acc[i] + a[i]);
    }

    let mut l = 0;
    let mut ans = 0;
    for r in 0..(n + 1) {
        while l < r && acc[r] - acc[l] > k {
            l += 1;
        }

        ans += r - l;
    }

    println!("{}", ans);
}
