use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: [usize; 9],
        b: [usize; 8],
    };

    let takahashi: usize = a.iter().sum();
    let aoki: usize = b.iter().sum();

    if takahashi < aoki {
        println!("0");
    } else {
        println!("{}", takahashi - aoki + 1);
    }
}
