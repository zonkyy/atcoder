use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let mut ans = 0;
    for c in s {
        ans *= 26;
        let i = (c as u32) - ('A' as u32) + 1;
        ans += i as u128;
    }

    println!("{:?}", ans);
}
