use num_integer::lcm;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: u128,
        m: u128,
        k: u128,
    };

    let nm_lcm = lcm(n, m);
    let cnt_between_lcm = (nm_lcm / n - 1) + (nm_lcm / m - 1);

    let skip = k / cnt_between_lcm;

    let mut rank = cnt_between_lcm * skip;
    let mut num = nm_lcm * skip;
    let (mut ni, mut mi) = (num / n + 1, num / m + 1);
    while rank < k {
        if n * ni < m * mi {
            num = n * ni;
            ni += 1;
        } else {
            num = m * mi;
            mi += 1;
        }
        rank += 1;
    }

    if k % cnt_between_lcm == 0 {
        num -= n.min(m);
    }

    println!("{}", num);
}
