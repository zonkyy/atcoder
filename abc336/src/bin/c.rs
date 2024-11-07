use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut n: usize,
    };
    n -= 1;

    if n == 0 {
        println!("0");
        return;
    }

    let mut digit = 1;
    while digit < n {
        digit *= 5;
    }

    let mut v = Vec::new();
    while digit >= 1 {
        v.push(n / digit);
        n %= digit;
        digit /= 5;
    }

    for i in 1..v.len() {
        print!("{}", v[i] * 2);
    }
    println!();
}
