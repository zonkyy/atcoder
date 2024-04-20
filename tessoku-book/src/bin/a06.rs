use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    };

    let mut ps = vec![0];
    for i in 0..n {
        ps.push(ps[i] + a[i]);
    }

    for (l, r) in lr {
        println!("{}", ps[r] - ps[l - 1]);
    }
}
