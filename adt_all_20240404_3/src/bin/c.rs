use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    };

    let mut names = Vec::new();
    for i in 0..k {
        names.push(s[i].clone());
    }

    names.sort();

    for n in names.iter() {
        println!("{}", n);
    }
}
