use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: f64,
    };

    let mut l = 0.001;
    let mut r = 100.0;
    while r - l >= 0.0001 {
        let mid = (l + r) / 2.0;
        let val = mid * mid * mid + mid;
        if val < n {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", l);
}
