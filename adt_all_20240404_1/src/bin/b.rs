use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    };

    if x >= y {
        println!("0");
        return;
    }

    let mut ans = (y - x) / 10;
    if (y - x) % 10 != 0 {
        ans += 1;
    }
    println!("{}", ans);
}
