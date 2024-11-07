use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        st: [(usize, usize); n-1],
    };

    let mut ex = 0;
    for i in 0..(n - 1) {
        ex = ((a[i] + ex) / st[i].0) * st[i].1;
    }

    println!("{}", a[n - 1] + ex);
}
