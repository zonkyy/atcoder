use num_integer::gcd;
use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    let ab = gcd(a, b);
    let abc = gcd(ab, c);

    println!("{}", a / abc + b / abc + c / abc - 3);
}
