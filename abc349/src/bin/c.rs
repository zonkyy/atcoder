use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut si = 0;
    let mut ti = 0;
    while si < s.len() && ti < t.len() {
        if s[si].to_uppercase().next().unwrap() as char == t[ti] {
            si += 1;
            ti += 1;
        } else {
            si += 1;
        }
    }

    if ti == 3 {
        println!("Yes");
    } else if ti == 2 && t.last().unwrap() == &'X' {
        println!("Yes");
    } else {
        println!("No");
    }
}
