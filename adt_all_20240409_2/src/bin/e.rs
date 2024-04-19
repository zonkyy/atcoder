use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut n: i64,
    };

    if n == 1 {
        println!("0");
        return;
    }
    n -= 1;

    let mut digits = vec![1];
    let mut d = 1;
    while d <= 10i64.pow(12) {
        d *= 5;
        digits.push(d);
    }
    digits.reverse();

    let mut ans_digits = Vec::new();
    for d in digits {
        let mut t = 0;
        while n - d >= 0 {
            n -= d;
            t += 1;
        }
        if !(t == 0 && ans_digits.is_empty()) {
            ans_digits.push(t);
        }
    }

    for i in ans_digits.iter() {
        print!("{:?}", i * 2);
    }
    println!();
}
