use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: Chars,
    };

    for cc in c {
        print!("{}{}", cc, cc);
    }
    println!();
}
