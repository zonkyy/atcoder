use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [usize; n],
    };

    for (idx, &i) in h.iter().enumerate() {
        if h[0] < i {
            println!("{}", idx + 1);
            return;
        }
    }

    println!("-1");
}
