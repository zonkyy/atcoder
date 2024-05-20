use num_integer::div_ceil;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        m: usize,
        l: usize,
    };

    let mut ans = usize::MAX;
    for li in 0..=(n / 12 + 1) {
        for mi in 0..=(n / 8 + 1) {
            let mut si = 0;
            if n > li * 12 + mi * 8 {
                si = div_ceil(n - li * 12 - mi * 8, 6);
            }
            ans = ans.min(l * li + m * mi + s * si);
        }
    }

    println!("{}", ans);
}
