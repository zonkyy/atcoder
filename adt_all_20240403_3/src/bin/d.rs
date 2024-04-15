use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
        mut t: Chars,
    };

    if s == t {
        println!("Yes");
        return;
    }

    for i in 0..(t.len() - 1) {
        t.swap(i, i + 1);
        if s == t {
            println!("Yes");
            return;
        }
        t.swap(i, i + 1);
    }

    println!("No");
}
