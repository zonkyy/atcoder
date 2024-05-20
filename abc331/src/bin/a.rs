use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        m: usize,
        d: usize,
        mut yy: usize,
        mut mm: usize,
        mut dd: usize,
    };

    dd += 1;

    if dd > d {
        mm += 1;
        dd = 1;
    }

    if mm > m {
        yy += 1;
        mm = 1;
    }

    println!("{} {} {}", yy, mm, dd);
}
