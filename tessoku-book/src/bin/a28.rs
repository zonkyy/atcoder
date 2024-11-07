use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        ta: [(char, usize); n],
    };

    let mut ans = 0;
    for &(t, a) in ta.iter() {
        match t {
            '+' => {
                ans += a;
                ans %= 10000;
            }
            '-' => {
                if ans < a {
                    ans += 10000;
                }
                ans -= a;
            }
            '*' => {
                ans *= a;
                ans %= 10000;
            }
            _ => unreachable!(),
        }
        println!("{}", ans);
    }
}
