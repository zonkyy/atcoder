use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut sc: [(String, usize); n],
    };
    sc.sort();

    let mut sum = 0;
    for (s, c) in sc.iter() {
        sum += c;
    }
    println!("{}", sc[sum % n].0);
}
