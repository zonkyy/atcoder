use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
    };

    let mut plant = 0;
    for i in 0.. {
        plant += 2usize.pow(i);
        if plant > h {
            println!("{}", i + 1);
            return;
        }
    }
}
