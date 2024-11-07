use std::cmp::min;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut a: Chars,
        mut b: Chars,
    };
    a.reverse();
    b.reverse();

    for i in (0..(min(a.len(), b.len()))) {
        let a_num = a[i] as u32 - '0' as u32;
        let b_num = b[i] as u32 - '0' as u32;
        if a_num + b_num >= 10 {
            println!("Hard");
            return;
        }
    }

    println!("Easy");
}
