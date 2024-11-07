use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        t: Chars,
    };

    for c in t {
        match c {
            '1' => print!("{}", s1),
            '2' => print!("{}", s2),
            '3' => print!("{}", s3),
            _ => unreachable!(),
        };
    }
    println!()
}
