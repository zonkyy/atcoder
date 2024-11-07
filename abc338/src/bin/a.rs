use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    if !('A'..='Z').contains(&s[0]) {
        println!("No");
        return;
    }

    for i in 1..(s.len()) {
        if !('a'..='z').contains(&s[i]) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
