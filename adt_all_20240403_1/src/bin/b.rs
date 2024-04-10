use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    };

    if vec![1, 2, 4, 5, 7, 8].contains(&a) && a + 1 == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
