use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut buf = 0;
    let mut ans = 1;
    for i in a {
        buf += i;
        if buf > k {
            ans += 1;
            buf = i;
        }
    }

    if buf > k {
        ans += 1;
    }

    println!("{}", ans);
}
