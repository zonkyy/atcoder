use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut now = 0;
    for i in a {
        now += i;
        if now < 0 {
            now = 0;
        }
    }

    println!("{}", now);
}
