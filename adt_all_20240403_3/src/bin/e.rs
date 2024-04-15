use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut ans: u128 = 0;
    let mut right_pos: u128 = 0;
    for i in 0..n {
        if a[i] == i {
            right_pos += 1;
        } else if a[a[i]] == i {
            ans += 1;
        }
    }

    ans /= 2;
    ans += right_pos * (right_pos - 1) / 2;
    println!("{}", ans);
}
