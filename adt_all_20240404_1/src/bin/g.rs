use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        q: usize,
    };
    let MOD: i64 = 998244353;

    let mut v = vec![1];
    let mut ans: i64 = 1;
    let mut digit: i64 = 1;
    for _ in 0..q {
        input! {
            rule: usize,
        }
        match rule {
            2 => {
                digit -= 1;

                let mut tmp = 1;
                for i in 0..digit {
                    tmp = tmp * 10 % MOD;
                }
                ans -= tmp;
                if ans < 0 {
                    ans += MOD;
                }
            }
            3 => {
                println!("{}", ans);
            }
            _ => {
                input! {
                    n: i64,
                }
                ans = (ans * 10 + n) % MOD;
                digit += 1;
            }
        }
    }
}
