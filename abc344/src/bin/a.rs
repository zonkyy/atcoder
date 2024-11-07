use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let mut out = true;
    for c in s {
        if c == '|' {
            out = !out;
        }

        if out && c != '|' {
            print!("{}", c);
        }
    }
}
