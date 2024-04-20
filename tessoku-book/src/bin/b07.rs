use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    };

    const UPPER: usize = 500009;
    let mut v = vec![0; UPPER];
    for (l, r) in lr {
        v[l] += 1;
        v[r] -= 1;
    }

    let mut ans = 0;
    for i in 0..t {
        ans += v[i];
        println!("{}", ans);
    }
}
