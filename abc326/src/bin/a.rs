use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        x: i64,
        y: i64,
    };

    if y > x + 2 || y < x - 3 {
        println!("No");
    } else {
        println!("Yes");
    }
}
