use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };

    let mut diff_max = 0;
    let mut sum = 0;
    for (a, b) in ab {
        sum += a;
        diff_max = diff_max.max(b - a);
    }

    println!("{}", sum + diff_max);
}
