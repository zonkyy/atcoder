use num_integer::div_floor;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut a: i128,
        mut b: i128,
        mut c: i128,
        mut d: i128,
    };
    // 都合のいい左下を決定 (a2: 4の倍数、b2: 2の倍数)
    let a2 = div_floor(a, 4) * 4;
    let b2 = div_floor(b, 2) * 2;

    let w = c - a2;
    let h = d - b2;
    let mut ans = 0;
    ans += (w/ 4) * 4 * h;

    if c % 4 > 0 {
        ans += ( / 2) * 3;
        if d % 2 == 1 {
            ans += 2;
        }
    }
    println!("{}", ans);
    if c % 4 > 1 {
        ans += (d / 2) * 3;
        if d % 2 == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
    if c % 4 > 2 {
        ans += d / 2;
    }

    println!("{}", ans);
}
