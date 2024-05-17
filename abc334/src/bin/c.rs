use std::cmp;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k]
    };

    if k == 1 {
        println!("0");
        return;
    }

    let mut lv = vec![0; k / 2];
    let mut acc = 0;
    for i in 0..(k / 2) {
        acc += a[i * 2 + 1] - a[i * 2];
        lv[i] = acc;
    }

    if k % 2 == 0 {
        println!("{}", acc);
        return;
    }

    let mut rv = vec![0; k / 2];
    acc = 0;
    for i in 0..(k / 2) {
        acc += a[k - i * 2 - 1] - a[k - i * 2 - 2];
        rv[k / 2 - i - 1] = acc;
    }

    let mut ans = *cmp::min(lv.last(), rv.first()).unwrap();
    for i in 0..(k / 2 - 1) {
        ans = ans.min(lv[i] + rv[i + 1]);
    }

    println!("{}", ans);
}

// 1 2 4 5 7 8 12 15 16 19 20

//  1   1   1    3     3
//  1   2   3    6     9

//    2   2   4     1     1
//   10   8   6     2     1
