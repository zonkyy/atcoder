use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    println!(
        "{}",
        a.iter()
            .filter(|&x| x != a.iter().max().unwrap())
            .max()
            .unwrap()
    );
}
