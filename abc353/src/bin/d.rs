use im_rc::HashMap;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    const MOD: usize = 998244353;

    let mut digit = vec![0; n];
    for i in 0..n {
        digit[i] = a[i].to_string().len();
    }

    let mut d_cnt = vec![0; 11];
    for i in 0..n {
        d_cnt[digit[i]] += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        d_cnt[a[i].to_string().len()] -= 1;
        let mut tmp = 0;
        for di in (1..11).rev() {
            tmp += d_cnt[di] % MOD * a[i] % MOD;
            tmp = tmp % MOD;
            tmp *= 10;
            tmp = tmp % MOD;
        }

        ans += tmp % MOD;
        ans += a[i] * i % MOD;
    }

    println!("{}", ans % MOD);
}
