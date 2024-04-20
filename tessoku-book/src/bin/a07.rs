use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(Usize1, Usize1); n],
    };

    const UPPER: usize = 10usize.pow(5);
    let mut v: Vec<i64> = vec![0; UPPER];
    for (l, r) in lr {
        v[l] += 1;
        v[r + 1] -= 1;
    }

    let mut ans = 0;
    for i in 0..d {
        ans += v[i];
        println!("{}", ans);
    }
}
