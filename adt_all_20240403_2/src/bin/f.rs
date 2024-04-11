use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    };

    let mut si = 0;
    let mut ti = 0;
    while si < n {
        if s[si] == t[ti] {
            println!("Yes");
            si += 1;
            ti += 1;
        } else {
            println!("No");
            si += 1;
        }
    }
}
