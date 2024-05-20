use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(Usize1, Usize1); q],
    };

    let mut acc = vec![0];
    let mut acctmp = 0;
    for i in 1..n {
        if s[i - 1] == s[i] {
            acctmp += 1;
        }
        acc.push(acctmp);
    }

    for &(l, r) in lr.iter() {
        println!("{}", acc[r] - acc[l]);
    }
}
