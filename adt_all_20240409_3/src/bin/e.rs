use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let upper = 2 * n + 9;
    let mut v = vec![0; upper];
    for (i, j) in a.iter().enumerate() {
        v[2 * (i + 1)] = v[*j] + 1;
        v[2 * (i + 1) + 1] = v[*j] + 1;
    }

    for i in 1..(2 * n + 2) {
        println!("{}", v[i]);
    }
}
