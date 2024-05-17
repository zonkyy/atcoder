use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut n: usize,
    };

    let mut ans = 0;
    while n % 2 == 0 {
        ans += 1;
        n /= 2;
    }

    println!("{}", ans);
}
