use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: [String; n],
    };

    for s in w {
        if ["and", "not", "that", "the", "you"].contains(&s.as_str()) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
