use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        y: usize,
    };

    let ans = match y % 4 {
        0 => y + 2,
        1 => y + 1,
        2 => y,
        3 => y + 3,
        _ => unreachable!(),
    };
    println!("{}", ans);
}
