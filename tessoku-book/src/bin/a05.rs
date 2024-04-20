use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: i64,
        k: i64,
    };

    let mut ans = 0;
    for a in 1..=n {
        for b in 1..=n {
            let c = k - a - b;
            if 1 <= c && c <= n {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
