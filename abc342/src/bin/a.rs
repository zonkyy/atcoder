use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let mut v = Vec::new();
    for i in 0..s.len() {
        if s[i] != s[0] {
            v.push(i);
        }
    }

    if v.len() >= 2 {
        println!("1");
    } else {
        println!("{}", v[0] + 1);
    }
}
