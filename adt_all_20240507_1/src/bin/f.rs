use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let mut ans: usize = 0;
    for c in s {
        ans *= 26;
        ans += (c as u8 - 'A' as u8 + 1) as usize;
    }

    println!("{}", ans);
}
