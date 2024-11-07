use std::cmp;

use num::Integer;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: i64,
        m: i64,
        mut l: i64,
        mut r: i64,
    };
    l -= a;
    r -= a;

    // 範囲に入っている最左端の 1 つ左
    let nl = (l - 1).div_euclid(m);
    // 範囲に入っている最右端
    let nr = r.div_euclid(m);

    println!("{}", nr - nl);
}
