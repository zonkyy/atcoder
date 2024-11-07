use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    };

    let mut t = 0;
    let mut a = 0;
    for (x, y) in xy {
        t += x;
        a += y;
    }

    if t > a {
        println!("Takahashi");
    } else if t < a {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
