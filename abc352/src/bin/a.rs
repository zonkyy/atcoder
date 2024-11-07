use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
    };

    let (mut start, mut end) = (x, y);
    if x > y {
        start = y;
        end = x;
    }

    for i in start..=end {
        if i == z {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
