use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut ans = Vec::new();
    let mut si = 0;
    let mut ti = 0;
    while si < s.len() && ti < t.len() {
        if s[si] == t[ti] {
            ans.push((ti + 1).to_string());
            si += 1;
        }

        ti += 1;
    }

    println!("{}", ans.join(" "));
}
