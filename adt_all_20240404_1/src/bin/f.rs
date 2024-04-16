use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(usize, usize); q],
    };

    let mut idx = 0;
    for (t, x) in queries {
        if t == 1 {
            idx = (idx + n) - x;
        } else {
            println!("{}", s[(idx + x - 1) % n]);
        }
    }
}
