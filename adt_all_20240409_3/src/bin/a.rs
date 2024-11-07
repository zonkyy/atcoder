use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        c: (i64, i64),
    };

    let x;
    if a.0 == b.0 {
        x = c.0;
    } else if a.0 == c.0 {
        x = b.0;
    } else {
        x = a.0;
    }

    let y;
    if a.1 == b.1 {
        y = c.1;
    } else if a.1 == c.1 {
        y = b.1;
    } else {
        y = a.1;
    }

    println!("{} {}", x, y);
}
