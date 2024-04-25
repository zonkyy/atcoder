use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        t: [Usize1; q],
    };

    let mut v = vec![true; n];
    for i in t {
        v[i] = !v[i];
    }

    let mut ans = 0;
    for b in v {
        if b {
            ans += 1;
        }
    }

    println!("{}", ans);
}
