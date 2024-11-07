use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
    };

    if s.contains("ab") {
        println!("Yes");
    } else if s.contains("ba") {
        println!("Yes");
    } else {
        println!("No");
    }
}
