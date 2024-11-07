use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q],
    };
    a.sort();

    for i in 0..q {
        println!("{}", a.partition_point(|&y| y < x[i]));
    }
}
