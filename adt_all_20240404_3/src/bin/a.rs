use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut v: i64,
        a: i64,
        b: i64,
        c: i64,
    };

    loop {
        v -= a;
        if v < 0 {
            println!("F");
            break;
        }

        v -= b;
        if v < 0 {
            println!("M");
            break;
        }

        v -= c;
        if v < 0 {
            println!("T");
            break;
        }
    }
}
