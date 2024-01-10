use num_integer::gcd;
use proconio::input;

fn main() {
    input! {
        a : u64,
        b : u64,
    }

    let gcd = gcd(a, b);
    match a.checked_mul(b / gcd) {
        Some(x) => {
            if x <= 10u64.pow(18u32) {
                println!("{}", x);
            } else {
                println!("Large");
            }
        }
        None => {
            println!("Large")
        }
    }
}
