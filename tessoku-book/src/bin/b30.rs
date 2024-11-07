use ac_library::ModInt1000000007 as Mint;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    };

    let mut numer = Mint::new(1);
    for i in h..=(h + w - 2) {
        numer *= i;
    }

    let mut denom = Mint::new(1);
    for i in 1..=(w - 1) {
        denom *= i;
    }

    println!("{}", numer * denom.inv());
}
