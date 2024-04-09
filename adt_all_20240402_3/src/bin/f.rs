use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u128,
        mut ab: [(u128, u128); n],
    };

    let mut max: u128 = ab.iter().map(|&(_, b)| b as u128).sum();
    ab.sort_by(|a, b| a.0.cmp(&b.0));

    if max <= k {
        println!("1");
        return;
    }

    for (a, b) in ab {
        max -= b;
        if max <= k {
            println!("{}", a + 1);
            return;
        }
    }
}
